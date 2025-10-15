<template>
  <div>
    <label v-bind:for="id" class="text-gray-700">{{ label }}</label>
    <textarea
      v-model="model"
      v-bind:id="id"
      v-bind:placeholder="placeholder"
      rows="4"
      class="rounded mt-1 block w-full"
    />
  </div>
</template>

<script setup lang="ts">
defineProps({
  label: {
    type: String,
    required: true,
  },
  id: {
    type: String,
    required: true,
  },
  placeholder: {
    type: String,
    required: true,
  },
})
const model = defineModel<string>({ required: true })
</script>
