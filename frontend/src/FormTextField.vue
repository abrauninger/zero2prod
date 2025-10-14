<template>
  <div>
    <label v-bind:for="id" class="text-gray-700">{{ label }}</label>
    <input
      type="text"
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
})
const model = defineModel()
</script>
