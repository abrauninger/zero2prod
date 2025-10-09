<template>
  <h1>Publish a new newsletter issue</h1>
  <form @submit.prevent="handleSubmit">
    <div>
      <label for="title">Title:</label>
      <input type="text" id="title" v-model="title" placeholder="Enter newsletter title" />
    </div>

    <div>
      <label for="name">Plain-text content:</label>
      <input
        type="text"
        id="content_text"
        v-model="content_text"
        placeholder="Enter plain-text content"
      />
    </div>

    <div>
      <label for="name">HTML content:</label>
      <input
        type="text"
        id="content_html"
        v-model="content_html"
        placeholder="Enter HTML content"
      />
    </div>

    <div>
      <button type="submit">Subscribe</button>
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
const content_text = ref('')
const content_html = ref('')
const errorMessage: Ref<string | null> = ref(null)
const infoMessage: Ref<string | null> = ref(null)

const idempotency_key = uuidv4()

const handleSubmit = async () => {
  publishNewsletter(title.value, content_text.value, content_html.value, idempotency_key, {
    error: errorMessage,
    info: infoMessage,
  })
}
</script>
