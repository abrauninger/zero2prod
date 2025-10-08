<template>
  <h1>Log in</h1>
  <form @submit.prevent="handleSubmit">
    <div>
      <label for="username">Username:</label>
      <input type="text" id="username" v-model="username" placeholder="Enter your username" />
    </div>

    <div>
      <label for="passwrd">Password:</label>
      <input type="password" id="password" v-model="password" placeholder="Enter your password" />
    </div>

    <div>
      <button type="submit">Log in</button>
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
import { login } from './api.ts'

export default {
  data() {
    return {
      username: '',
      password: '',
      errorMessage: null as string | null,
      infoMessage: null as string | null,
    }
  },
  methods: {
    async handleSubmit() {
      this.errorMessage = null
      this.infoMessage = null

      try {
        const response = await login(this.username, this.password)

        console.log('Response received!')

        if (!response.ok) {
          const responseContent = await response.json()
          console.log(responseContent)
          this.errorMessage = error_message(responseContent.error_id)
        } else {
          // TODO: Route back to whatever the user originally tried
          this.$router.push('/admin')
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
    case 'invalid_credentials': {
      return 'The username and password that you entered did not work. Try again with different credentials.'
    }
    case 'internal_error': {
      return 'An internal error occurred, and we were unable to log you in. Apologies for the inconvenience.'
    }
  }

  console.log(`Unrecognized error ID: ${error_id}`)
  return 'Submission failed'
}
</script>
