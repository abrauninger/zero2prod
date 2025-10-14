<template>
  <AppForm heading="Publish a new newsletter issue" @submit="handleSubmit">
    <FormTextField v-model="title" id="title" label="Title" placeholder="Enter newsletter title" />
    <FormTextField
      v-model="contentText"
      id="contentText"
      label="Plain-text content"
      placeholder="Enter plain-text content"
    />
    <FormTextField
      v-model="contentHtml"
      id="contentHtml"
      label="HTML content"
      placeholder="Enter HTML content"
    />

    <SubmitButton>Publish</SubmitButton>

    <div v-if="errorMessage" class="error-message">
      {{ errorMessage }}
    </div>

    <div v-if="infoMessage" class="info-message">
      {{ infoMessage }}
    </div>
  </AppForm>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { Ref } from 'vue'
import { v4 as uuidv4 } from 'uuid'

import AppForm from './AppForm.vue'
import FormTextField from './FormTextField.vue'
import SubmitButton from './SubmitButton.vue'

import { publishNewsletter } from './api.ts'

const title = ref('')
const contentText = ref('')
const contentHtml = ref('')
const errorMessage: Ref<string | null> = ref(null)
const infoMessage: Ref<string | null> = ref(null)

const idempotency_key = uuidv4()

const handleSubmit = async () => {
  // TODO: Handle 401 'unauthorized' in case user navigates directly to this form
  publishNewsletter(title.value, contentText.value, contentHtml.value, idempotency_key, {
    error: errorMessage,
    info: infoMessage,
  })
}
</script>
