<template>
  <div v-if="errorMessage" class="error-message">
    {{ errorMessage }}
  </div>

  <div v-if="infoMessage" class="info-message">
    {{ infoMessage }}
  </div>
</template>

<script setup lang="ts">
import type { PropType } from 'vue'

defineProps({
  errorMessage: {
    type: String as PropType<string | null>,
    default: null,
  },
  infoMessage: {
    type: String as PropType<string | null>,
    default: null,
  },
})
</script>
