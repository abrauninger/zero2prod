<template>
  <div class="mx-auto max-w-xl py-12">
    <h1 class="text-4xl font-bold">Welcome to our newsletter</h1>
    <form @submit.prevent="handleSubmit">
      <div class="grid grid-cols-1 gap-6 mt-8">
        <div>
          <label for="name" class="text-gray-700">Name</label>
          <input
            type="text"
            id="name"
            v-model="name"
            placeholder="Enter your name"
            class="rounded mt-1 block w-full"
          />
        </div>

        <div>
          <label for="email" class="text-gray-700">Email address</label>
          <input
            type="text"
            id="email"
            v-model="email"
            placeholder="Enter your email address"
            class="rounded mt-1 block w-full"
          />
        </div>

        <SubmitButton label="Subscribe" />

        <div v-if="errorMessage" class="error-message">
          {{ errorMessage }}
        </div>

        <div v-if="infoMessage" class="info-message">
          {{ infoMessage }}
        </div>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'
import { addSubscriber } from './api.ts'
import SubmitButton from './SubmitButton.vue'

const name = ref('')
const email = ref('')
const errorMessage: Ref<string | null> = ref(null)
const infoMessage: Ref<string | null> = ref(null)

const handleSubmit = async () => {
  addSubscriber(name.value, email.value, { error: errorMessage, info: infoMessage })
}
</script>
