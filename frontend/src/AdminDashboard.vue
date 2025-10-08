<template>
  <h1>This is the admin dashboard</h1>
  <p>You are logged in as: {{ username }}</p>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { getUsername } from './api.ts'
import type { Ref } from 'vue'

const username: Ref<string | null> = ref(null)

watchEffect(async () => {
  const response = await getUsername()

  if (!response.ok) {
    username.value = "Couldn't get it!"
  } else {
    username.value = 'Got it!'
  }
})
</script>
