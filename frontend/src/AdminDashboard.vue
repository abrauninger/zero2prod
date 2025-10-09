<template>
  <h1>This is the admin dashboard</h1>
  <p>
    You are logged in as: <strong>{{ username }}</strong>
  </p>
  <p>Available actions:</p>
  <ol>
    <li><RouterLink to="/admin/newsletters">Send a newsletter issue</RouterLink></li>
    <li><RouterLink to="/admin/password">Change password</RouterLink></li>
  </ol>
  <button @click="logout">Log out</button>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { useRouter } from 'vue-router'
import { getUsername, logout as logoutApi } from './api.ts'
import type { Ref } from 'vue'

const username: Ref<string | null> = ref(null)
const router = useRouter()

watchEffect(async () => {
  const response = await getUsername()

  if (response.ok) {
    const responseContent = await response.json()
    console.log(responseContent)
    username.value = responseContent.username
  } else {
    if (response.status != 401) {
      console.log('Unexpected error fetching username')
    }
    router.replace('/login')
  }
})

async function logout() {
  await logoutApi()
  router.replace('/')
}
</script>
