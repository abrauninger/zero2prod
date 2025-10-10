<template>
  <h1>Publish a new newsletter issue</h1>
  <form @submit.prevent="handleSubmit">
    <div>
      <label for="title">Title:</label>
      <input type="text" id="title" v-model="title" placeholder="Enter newsletter title" />
    </div>

    <div>
      <label for="contentText">Plain-text content:</label>
      <input
        type="text"
        id="contentText"
        v-model="contentText"
        placeholder="Enter plain-text content"
      />
    </div>

    <div>
      <label for="contentHtml">HTML content:</label>
      <input type="text" id="contentHtml" v-model="contentHtml" placeholder="Enter HTML content" />
    </div>

    <div>
      <button type="submit">Publish</button>
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
import { v4 as uuidv4 } from 'uuid'

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
