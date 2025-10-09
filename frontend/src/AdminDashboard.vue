<template>
  <h1>This is the admin dashboard</h1>
  <p>You are logged in as: {{ username }}</p>
  <p>Available actions:</p>
  <ol>
    <li><RouterLink to="/admin/newsletters">Send a newsletter issue</RouterLink></li>
    <!-- <li><RouterLink to="/admin/password">Change password</RouterLink></li> -->
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

  if (!response.ok) {
    if (response.status == 401) {
      router.replace('/login')
    } else {
      // TODO: Temp
      username.value = "Couldn't get it!"
    }
  } else {
    const responseContent = await response.json()
    console.log(responseContent)
    username.value = responseContent.username
  }
})

async function logout() {
  await logoutApi()
  router.replace('/')
}
</script>
