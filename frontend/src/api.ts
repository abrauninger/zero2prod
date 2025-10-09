import type { Ref } from 'vue'

interface Messages {
  error: Ref<string | null>
  info: Ref<string | null>
}

export function getUsername() {
  return fetch('/api/admin/user', {
    method: 'GET',
  })
}

export function login(username: string, password: string, messages: Messages): Promise<boolean> {
  return fetchWithPost('/api/login', { username, password }, messages, null)
}

export function logout() {
  return fetch('/api/admin/logout', {
    method: 'GET',
  })
}

export function addSubscriber(name: string, email: string, messages: Messages) {
  return fetchWithPost(
    '/api/subscriptions',
    { name, email },
    messages,
    "You have subscribed to our newsletter. Stay tuned, you're going to love it!",
  )
}

export function publishNewsletter(
  title: string,
  contentText: string,
  contentHtml: string,
  idempotencyKey: string,
  messages: Messages,
) {
  return fetchWithPost(
    '/api/admin/newsletters',
    {
      title,
      content_text: contentText,
      content_html: contentHtml,
      idempotency_key: idempotencyKey,
    },
    messages,
    'Your newsletter publish request has been accepted, and emails will go out shortly.',
  )
}

export function changePassword(
  currentPassword: string,
  newPassword: string,
  newPasswordCheck: string,
  messages: Messages,
) {
  // TODO: Verify the two new password match on the client side without sending to the API.
  return fetchWithPost(
    '/api/admin/password',
    {
      current_password: currentPassword,
      new_password: newPassword,
      new_password_check: newPasswordCheck,
    },
    messages,
    'Your password has been changed.',
  )
}

async function fetchWithPost(
  url: string,
  body: object,
  messages: Messages,
  successMessage: string | null,
): Promise<boolean> {
  messages.error.value = null
  messages.info.value = null

  console.log(`Fetching POST ${url}`)

  try {
    const response = await fetch(url, {
      method: 'POST',
      body: JSON.stringify(body),
      headers: {
        'Content-Type': 'application/json',
      },
    })

    console.log(`Response received from POST ${url}`)

    if (response.ok) {
      messages.info.value = successMessage
      return true
    }
    const responseContent = await response.json()
    console.log(responseContent)
    messages.error.value = errorMessage(responseContent.error_id)
  } catch (error: unknown) {
    if (error instanceof Error) {
      messages.error.value =
        'An internal front-end error has occured. Apologies for the inconvenience.'
    }
    console.error(`Error while fetching POST ${url}: ${error}`)
  }

  return false
}

function errorMessage(errorId: string): string {
  // TODO: Put success messages in a similar function
  switch (errorId) {
    case 'invalid_credentials': {
      return 'The username and password that you entered did not work. Try again with different credentials.'
    }
    case 'invalid_data': {
      return 'There was a problem with the form data you entered. Please try again.'
    }
    case 'send_confirmation_email': {
      return 'We were unable to send a confirmation email to that email address.'
    }
    case 'internal_error': {
      return 'An internal error occurred, and we were unable to add you to our subscription list. Apologies for the inconvenience.'
    }
  }

  console.log(`Unrecognized error ID: ${errorId}`)
  return 'Submission failed'
}
