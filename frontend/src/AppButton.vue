<template>
  <button
    @click="$emit('click')"
    class="text-white bg-blue-500 hover:bg-blue-800 font-medium text-sm rounded-lg px-5 py-2.5"
  >
    <slot />
  </button>
</template>

<script setup lang="ts">
defineEmits(['click'])
</script>
