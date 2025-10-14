<template>
  <AppForm heading="Welcome to our newsletter" @submit="handleSubmit">
    <FormTextField v-model="name" id="name" label="Name" placeholder="Enter your name" />
    <FormTextField
      v-model="email"
      id="email"
      label="Email address"
      placeholder="Enter your email address"
    />

    <SubmitButton>Subscribe</SubmitButton>

    <div v-if="errorMessage" class="error-message">
      {{ errorMessage }}
    </div>

    <div v-if="infoMessage" class="info-message">
      {{ infoMessage }}
    </div>
  </AppForm>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'
import { addSubscriber } from './api.ts'
import AppForm from './AppForm.vue'
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
