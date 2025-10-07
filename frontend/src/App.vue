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
  </form>
</template>

<script lang="ts">
export default {
  data() {
    return {
      name: '',
      email: '',
      errorMessage: null as string | null,
      infoMessage: null,
    }
  },
  methods: {
    async handleSubmit() {
      this.errorMessage = null
      this.infoMessage = null

      try {
        const response = await fetch('/api/subscriptions', {
          method: 'POST',
          body: JSON.stringify({ name: this.name, email: this.email }),
          headers: {
            'Content-Type': 'application/json',
          },
        })

        const responseContent = await response.json()
        console.log(responseContent)

        if (!response.ok) {
          throw new Error(error_message(responseContent.error_id))
        }

        this.infoMessage = responseContent.message
      } catch (error: unknown) {
        if (error instanceof Error) {
          this.errorMessage = error.message
        }
        console.error('Error during submission: ', error)
      }
    },
  },
}

function error_message(error_id: string): string {
  switch (error_id) {
    case 'bad_subscription_form_data': {
      return 'There was a problem with the form data you entered. Please try again.'
    }
    // TODO: Remove this one?
    case 'insert_subscriber': {
      return 'We were unable to add you as a subscriber. Apologies for the inconvenience.'
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

<style scoped>
.error-message {
  color: darkred;
  background-color: pink;
  border-style: solid;
  border-width: 1px;
  border-color: darkred;
  margin-top: 10px;
  padding: 10px;
}
</style>
