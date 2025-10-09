export function getUsername() {
  return fetch('/api/admin/user', {
    method: 'GET',
  })
}

export function login(username: string, password: string) {
  return fetch('/api/login', {
    method: 'POST',
    body: JSON.stringify({ username, password }),
    headers: {
      'Content-Type': 'application/json',
    },
  })
}

export function logout() {
  return fetch('/api/admin/logout', {
    method: 'GET',
  })
}

export function addSubscriber(name: string, email: string) {
  return fetch('/api/subscriptions', {
    method: 'POST',
    body: JSON.stringify({ name, email }),
    headers: {
      'Content-Type': 'application/json',
    },
  })
}

export function publishNewsletter(
  title: string,
  content_text: string,
  content_html: string,
  idempotency_key: string,
) {
  return fetch('/api/admin/newsletters', {
    method: 'POST',
    body: JSON.stringify({ title, content_text, content_html, idempotency_key }),
    headers: {
      'Content-Type': 'application/json',
    },
  })
}
