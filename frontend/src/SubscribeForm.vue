<template>
  <h1>Welcome to our newsletter</h1>
  <form @submit.prevent="handleSubmit">
    <div>
      <label for="name">Name:</label>
      <input type="text" id="name" v-model="name" placeholder="Enter your name" />
    </div>

    <div>
      <label for="email">Email address:</label>
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

<script setup lang="ts">
import { ref, type Ref } from 'vue'
import { addSubscriber } from './api.ts'

const name = ref('')
const email = ref('')
const errorMessage: Ref<string | null> = ref(null)
const infoMessage: Ref<string | null> = ref(null)

const handleSubmit = async () => {
  addSubscriber(name.value, email.value, { error: errorMessage, info: infoMessage })
}
</script>
