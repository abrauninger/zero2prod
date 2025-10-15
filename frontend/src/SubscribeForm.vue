<template>
  <AppForm heading="Welcome to our newsletter" @submit="handleSubmit">
    <p>To subscribe to our newsletter, enter your information here.</p>
    <FormTextField v-model="name" id="name" label="Name" placeholder="Enter your name" />
    <FormTextField
      v-model="email"
      id="email"
      label="Email address"
      placeholder="Enter your email address"
    />

    <SubmitButton>Subscribe</SubmitButton>

    <AppMessages v-bind:error-message="errorMessage" v-bind:info-message="infoMessage" />
  </AppForm>
</template>

<script setup lang="ts">
import { ref, type Ref } from 'vue'
import { addSubscriber } from './api.ts'
import AppForm from './AppForm.vue'
import AppMessages from './AppMessages.vue'
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
