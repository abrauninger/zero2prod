<template>
  <AppForm heading="Log in" @submit="handleSubmit">
    <FormTextField
      v-model="username"
      id="username"
      label="Username"
      placeholder="Enter your username"
    />
    <FormTextField
      v-model="password"
      type="password"
      id="password"
      label="Password"
      placeholder="Enter your password"
    />

    <SubmitButton>Log in</SubmitButton>

    <AppMessages v-bind:error-message="errorMessage" v-bind:info-message="infoMessage" />
  </AppForm>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { login } from './api.ts'
import type { Ref } from 'vue'
import { useRouter } from 'vue-router'

import AppForm from './AppForm.vue'
import AppMessages from './AppMessages.vue'
import FormTextField from './FormTextField.vue'
import SubmitButton from './SubmitButton.vue'

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
