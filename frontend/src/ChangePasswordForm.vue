<template>
  <AppForm heading="Change password" @submit="handleSubmit">
    <FormTextField
      v-model="currentPassword"
      id="currentPassword"
      label="Current password"
      placeholder="Enter current password"
      type="password"
    />
    <FormTextField
      v-model="newPassword"
      id="newPassword"
      label="Current password"
      placeholder="Enter new password"
      type="password"
    />
    <FormTextField
      v-model="newPasswordCheck"
      id="newPasswordCheck"
      label="Current password"
      placeholder="Enter new password"
      type="password"
    />

    <SubmitButton>Change password</SubmitButton>

    <AppMessages v-bind:error-message="errorMessage" v-bind:info-message="infoMessage" />
  </AppForm>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { Ref } from 'vue'

import { changePassword } from './api.ts'
import AppForm from './AppForm.vue'
import AppMessages from './AppMessages.vue'
import FormTextField from './FormTextField.vue'
import SubmitButton from './SubmitButton.vue'

const currentPassword = ref('')
const newPassword = ref('')
const newPasswordCheck = ref('')
const errorMessage: Ref<string | null> = ref(null)
const infoMessage: Ref<string | null> = ref(null)

const handleSubmit = async () => {
  // TODO: Handle 401 'unauthorized' in case user navigates directly to this form
  changePassword(currentPassword.value, newPassword.value, newPasswordCheck.value, {
    error: errorMessage,
    info: infoMessage,
  })
}
</script>
