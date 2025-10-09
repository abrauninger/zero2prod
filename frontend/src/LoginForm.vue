<template>
  <h1>Log in</h1>
  <form @submit.prevent="handleSubmit">
    <div>
      <label for="username">Username:</label>
      <input type="text" id="username" v-model="username" placeholder="Enter your username" />
    </div>

    <div>
      <label for="passwrd">Password:</label>
      <input type="password" id="password" v-model="password" placeholder="Enter your password" />
    </div>

    <div>
      <button type="submit">Log in</button>
    </div>

    <div v-if="errorMessage" class="error-message">
      {{ errorMessage }}
    </div>

    <div v-if="infoMessage" class="info-message">
      {{ infoMessage }}
    </div>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { login } from './api.ts'
import type { Ref } from 'vue'
import { useRouter } from 'vue-router'

const username = ref('')
const password = ref('')
const errorMessage: Ref<string | null> = ref(null)
const infoMessage: Ref<string | null> = ref(null)

const router = useRouter()

const handleSubmit = async () => {
  if (await login(username.value, password.value, { error: errorMessage, info: infoMessage })) {
    // TODO: Route back to whatever the user originally tried
    router.push('/admin')
  }
}
</script>
