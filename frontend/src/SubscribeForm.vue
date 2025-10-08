<script setup lang="ts"></script>

<template>
  <h1>Welcome to our newsletter</h1>
  <form @submit.prevent="handleSubmit">
    <div>
      <label for="name">Name:</label>
      <input type="text" id="name" v-model="name" placeholder="Enter your name" />
    </div>

    <div>
      <label for="name">Email address:</label>
      <input type="text" id="email" v-model="email" placeholder="Enter your email address" />
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

<script lang="ts">
import { addSubscriber } from './api.ts'

export default {
  data() {
    return {
      name: '',
      email: '',
      errorMessage: null as string | null,
      infoMessage: null as string | null,
    }
  },
  methods: {
    async handleSubmit() {
      this.errorMessage = null
      this.infoMessage = null

      try {
        const response = await addSubscriber(this.name, this.email)

        console.log('Response received!')

        if (!response.ok) {
          const responseContent = await response.json()
          console.log(responseContent)
          this.errorMessage = error_message(responseContent.error_id)
        } else {
          this.infoMessage =
            "You have subscribed to our newsletter. Stay tuned, you're going to love it!"
        }
      } catch (error: unknown) {
        if (error instanceof Error) {
          this.errorMessage =
            'An internal front-end error has occured. Apologies for the inconvenience.'
        }
        console.error('Error during submission: ', error)
      }
    },
  },
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
