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
  errorMessage.value = null
  infoMessage.value = null

  try {
    const response = await publishNewsletter(
      title.value,
      content_text.value,
      content_html.value,
      idempotency_key,
    )

    console.log('Response received!')

    if (!response.ok) {
      const responseContent = await response.json()
      console.log(responseContent)
      errorMessage.value = error_message(responseContent.error_id)
    } else {
      infoMessage.value =
        'Your newsletter publish request has been accepted, and emails will go out shortly.'
    }
  } catch (error: unknown) {
    if (error instanceof Error) {
      errorMessage.value =
        'An internal front-end error has occured. Apologies for the inconvenience.'
    }
    console.error('Error during submission: ', error)
  }
}

function error_message(error_id: string): string {
  switch (error_id) {
    case 'invalid_data': {
      return 'There was a problem with the form data you entered. Please try again.'
    }
    case 'send_confirmation_email': {
      return 'We were unable to send a confirmation email to that email address.'
    }
    case 'internal_error': {
      return 'An internal error occurred, and we were unable to add you to our subscription list. Apologies for the inconvenience.'
    }
  }

  console.log(`Unrecognized error ID: ${error_id}`)
  return 'Submission failed'
}
</script>
