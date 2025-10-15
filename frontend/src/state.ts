import { ref } from 'vue'
import type { Ref } from 'vue'
import type { Router } from 'vue-router'

import { getUsername, logout as logoutApi } from './api'

export const username: Ref<string | null> = ref(null)

// Route that we should return back to after login
export let loginSource: string | null = null

export const logout = async (router: Router) => {
  await logoutApi()
  username.value = null
  // TODO: Add a logged-out page as confirmation.
  router.push('/')
}

export const setLoginSource = (source: string) => {
  loginSource = source
}

export const fetchUsername = async () => {
  const response = await getUsername()

  if (response.ok) {
    const responseContent = await response.json()
    console.log(responseContent)
    username.value = responseContent.username
  } else {
    username.value = null
  }
}
