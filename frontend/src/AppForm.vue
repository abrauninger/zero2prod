<template>
  <div class="mx-auto max-w-xl py-12">
    <AppHeading>{{ heading }}</AppHeading>
    <form @submit.prevent="$emit('submit')">
      <div class="grid grid-cols-1 gap-6 mt-8">
        <slot />
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import AppHeading from './AppHeading.vue'

defineProps({
  heading: {
    type: String,
    required: true,
  },
})

defineEmits(['submit'])
</script>
