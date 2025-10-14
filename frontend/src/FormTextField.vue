<template>
  <div>
    <label v-bind:for="id" class="text-gray-700">{{ label }}</label>
    <input
      v-bind:type="type"
      v-model="model"
      v-bind:id="id"
      v-bind:placeholder="placeholder"
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
  type: {
    type: String,
    default: 'text',
  },
})
const model = defineModel()
</script>
