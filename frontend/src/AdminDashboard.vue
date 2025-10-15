<template>
  <div class="mx-auto max-w-xl py-12 px-12">
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
    </div>
  </div>
</template>

<script setup lang="ts">
import { watchEffect } from 'vue'
import { useRouter } from 'vue-router'
import { username } from './state.ts'

import AppHeading from './AppHeading.vue'

const router = useRouter()

watchEffect(async () => {
  if (username.value === null) {
    router.replace('/login')
  }
})
</script>
