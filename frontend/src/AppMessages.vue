<template>
  <div
    v-if="errorMessage"
    class="text-red-900 bg-red-200 border border-solid border-red-900 p-2 mt-2"
  >
    {{ errorMessage }}
  </div>

  <div
    v-if="infoMessage"
    class="text-green-900 bg-green-200 border border-solid border-green-900 p-2 mt-2"
  >
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
