<template>
  <button
    @click="$emit('click')"
    class="text-white bg-blue-700 hover:bg-blue-800 font-medium text-sm rounded-lg px-5 py-2.5 mt-8"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
defineEmits(['click'])
</script>
