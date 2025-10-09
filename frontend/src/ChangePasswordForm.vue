<template>
  <h1>Change password</h1>
  <form @submit.prevent="handleSubmit">
    <div>
      <label for="currentPassword">Current password:</label>
      <input
        type="password"
        id="currentPassword"
        v-model="currentPassword"
        placeholder="Enter current password"
      />
    </div>

    <div>
      <label for="newPassword">New password:</label>
      <input
        type="password"
        id="newPassword"
        v-model="newPassword"
        placeholder="Enter new password"
      />
    </div>

    <div>
      <label for="newPasswordCheck">Confirm new password:</label>
      <input
        type="password"
        id="newPasswordCheck"
        v-model="newPasswordCheck"
        placeholder="Enter the new password again"
      />
    </div>

    <div>
      <button type="submit">Change password</button>
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
import type { Ref } from 'vue'

import { changePassword } from './api.ts'

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
