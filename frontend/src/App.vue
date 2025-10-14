<template>
  <nav class="relative bg-gray-200 p-2">
    <div class="flex space-x-4">
      <RouterLink
        to="/"
        class="rounded-md px-3 py-2 text-sm font-medium bg-gray-300 text-black hover:bg-gray-400 hover:text-blue-800"
        >Subscribe</RouterLink
      >
      <RouterLink
        to="/admin"
        class="rounded-md px-3 py-2 text-sm font-medium bg-gray-300 text-black hover:bg-gray-400 hover:text-blue-800"
        >Admin dashboard</RouterLink
      >
    </div>
  </nav>
  <div>
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>
