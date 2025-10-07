use std::time::Duration;

use sqlx::{Executor, PgPool, Postgres, Transaction};
use tracing::{Span, field::display};
use uuid::Uuid;

use crate::{
    configuration::Settings, domain::SubscriberEmail, email_client::EmailClient,
    startup::get_connection_pool,
};

pub async fn run_worker_until_stopped(configuration: Settings) -> Result<(), anyhow::Error> {
    let connection_pool = get_connection_pool(&configuration.database);

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    worker_loop(connection_pool, email_client).await
}

enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

#[tracing::instrument(skip_all, fields(newsletter_issue_id=tracing::field::Empty, subscriber_email=tracing::field::Empty), err)]
async fn try_execute_task(
    pool: &PgPool,
    email_client: &EmailClient,
) -> Result<ExecutionOutcome, anyhow::Error> {
    let task = dequeue_task(pool).await?;
    let Some((transaction, issue_id, email)) = task else {
        return Ok(ExecutionOutcome::EmptyQueue);
    };

    Span::current()
        .record("newsletter_issue_id", display(issue_id))
        .record("subscriber_email", display(&email));

    match SubscriberEmail::parse(email.clone()) {
        Ok(email) => {
            let issue = get_issue(pool, issue_id).await?;
            if let Err(e) = email_client
                .send_email(
                    &email,
                    &issue.title,
                    &issue.content_html,
                    &issue.content_text,
                )
                .await
            {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Failed to deliver issue to a confirmed subscriber. Skipping.",
                );
            }
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Skipping a confirmed subscriber. Their stored contact details are invalid",
            );
        }
    }

    delete_task(transaction, issue_id, &email).await?;

    Ok(ExecutionOutcome::TaskCompleted)
}

type PgTransaction = Transaction<'static, Postgres>;

#[tracing::instrument(skip_all)]
async fn dequeue_task(
    pool: &PgPool,
) -> Result<Option<(PgTransaction, Uuid, String)>, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let row = sqlx::query!(
        r#"
        SELECT newsletter_issue_id, subscriber_email
        FROM issue_delivery_queue
        FOR UPDATE
        SKIP LOCKED
        LIMIT 1
        "#,
    )
    .fetch_optional(&mut *transaction)
    .await?;

    if let Some(row) = row {
        Ok(Some((
            transaction,
            row.newsletter_issue_id,
            row.subscriber_email,
        )))
    } else {
        Ok(None)
    }
}

#[tracing::instrument(skip_all)]
async fn delete_task(
    mut transaction: PgTransaction,
    issue_id: Uuid,
    email: &str,
) -> Result<(), anyhow::Error> {
    let query = sqlx::query!(
        r#"
        DELETE FROM issue_delivery_queue
        WHERE
            newsletter_issue_id = $1 AND
            subscriber_email = $2
        "#,
        issue_id,
        email
    );
    transaction.execute(query).await?;
    transaction.commit().await?;
    Ok(())
}

struct NewsletterIssue {
    title: String,
    content_text: String,
    content_html: String,
}

#[tracing::instrument(skip_all)]
async fn get_issue(pool: &PgPool, issue_id: Uuid) -> Result<NewsletterIssue, anyhow::Error> {
    let issue = sqlx::query_as!(
        NewsletterIssue,
        r#"
        SELECT title, content_text, content_html
        FROM newsletter_issues
        WHERE
            newsletter_issue_id = $1
        "#,
        issue_id
    )
    .fetch_one(pool)
    .await?;
    Ok(issue)
}

async fn worker_loop(pool: PgPool, email_client: EmailClient) -> Result<(), anyhow::Error> {
    loop {
        // TODO: p. 578: Don't sleep if 'try_execute_task' fails with a non-transient error
        match try_execute_task(&pool, &email_client).await {
            Ok(ExecutionOutcome::EmptyQueue) => {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Ok(ExecutionOutcome::TaskCompleted) => {}
        }
    }
}
