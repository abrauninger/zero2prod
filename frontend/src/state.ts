import { ref } from 'vue'
import type { Ref } from 'vue'
import { useRouter } from 'vue-router'

import { getUsername, logout as logoutApi } from './api'

export const username: Ref<string | null> = ref(null)

// Route that we should return back to after login
export let loginSource: string | null = null

export const logout = async () => {
  await logoutApi()
  username.value = null
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
