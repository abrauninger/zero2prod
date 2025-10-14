<template>
  <div class="mx-auto max-w-xl py-12">
    <h1 class="text-4xl font-bold">Welcome to our newsletter</h1>
    <form @submit.prevent="handleSubmit">
      <div class="grid grid-cols-1 gap-6 mt-8">
        <FormTextField v-model="name" id="name" label="Name" placeholder="Enter your name" />
        <FormTextField
          v-model="email"
          id="email"
          label="Email address"
          placeholder="Enter your email address"
        />

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
import FormTextField from './FormTextField.vue'
import SubmitButton from './SubmitButton.vue'

const name = ref('')
const email = ref('')
const errorMessage: Ref<string | null> = ref(null)
const infoMessage: Ref<string | null> = ref(null)

const handleSubmit = async () => {
  addSubscriber(name.value, email.value, { error: errorMessage, info: infoMessage })
}
</script>
