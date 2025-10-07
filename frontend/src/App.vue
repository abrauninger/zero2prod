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

        if (!response.ok) {
          throw new Error(responseContent.message || 'Submission failed.')
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
</script>

<style scoped>
.error-message {
  color: darkred;
  margin-top: 10px;
}
</style>
