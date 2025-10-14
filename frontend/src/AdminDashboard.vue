<template>
  <div class="mx-auto max-w-xl py-12">
    <AppHeading>This is the admin dashboard</AppHeading>
    <div class="mt-8">
      <p class="mb-4">
        You are logged in as: <strong>{{ username }}</strong>
      </p>
      <p class="mb-4">Available actions:</p>
      <ol class="list-decimal">
        <li>
          <RouterLink
            to="/admin/newsletters"
            class="text-blue-500 hover:text-blue-700 hover:underline"
            >Send a newsletter issue</RouterLink
          >
        </li>
        <li>
          <RouterLink to="/admin/password" class="text-blue-500 hover:text-blue-700 hover:underline"
            >Change password</RouterLink
          >
        </li>
      </ol>
      <AppButton @click="logout">Log out</AppButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { useRouter } from 'vue-router'
import { getUsername, logout as logoutApi } from './api.ts'
import type { Ref } from 'vue'

import AppHeading from './AppHeading.vue'
import AppButton from './AppButton.vue'

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
