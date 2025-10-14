<template>
  <div class="mx-auto max-w-xl py-12">
    <Heading>{{ heading }}</Heading>
    <form @submit.prevent="$emit('submit')">
      <div class="grid grid-cols-1 gap-6 mt-8">
        <slot />
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import Heading from './Heading.vue'

defineProps({
  heading: {
    type: String,
    required: true,
  },
})

defineEmits(['submit'])
</script>
