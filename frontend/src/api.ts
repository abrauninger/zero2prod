export function addSubscriber(name: string, email: string) {
  return fetch('/api/subscriptions', {
    method: 'POST',
    body: JSON.stringify({ name, email }),
    headers: {
      'Content-Type': 'application/json',
    },
  })
}
