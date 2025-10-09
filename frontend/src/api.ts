export function getUsername() {
  return fetch('/api/admin/user', {
    method: 'GET',
  })
}

export function login(username: string, password: string) {
  return fetchWithPost('/api/login', { username, password })
}

export function logout() {
  return fetch('/api/admin/logout', {
    method: 'GET',
  })
}

export function addSubscriber(name: string, email: string) {
  return fetchWithPost('/api/subscriptions', { name, email })
}

export function publishNewsletter(
  title: string,
  content_text: string,
  content_html: string,
  idempotency_key: string,
) {
  return fetchWithPost('/api/admin/newsletters', {
    title,
    content_text,
    content_html,
    idempotency_key,
  })
}

function fetchWithPost(url: string, body: object) {
  return fetch(url, {
    method: 'POST',
    body: JSON.stringify(body),
    headers: {
      'Content-Type': 'application/json',
    },
  })
}
