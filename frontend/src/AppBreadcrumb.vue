<template>
  <nav class="flex" aria-label="Breadcrumb">
    <ol class="inline-flex items-center space-x-1 md:space-x-3">
      <li v-for="(crumb, index) in breadcrumbs" :key="index" class="inline-flex items-center">
        <RouterLink v-if="crumb.link" :to="crumb.link">
          {{ crumb.name }}
        </RouterLink>
        <span v-else>
          {{ crumb.name }}
        </span>
        <ChevronRightIcon v-if="index < breadcrumbs.length - 1" />
      </li>
    </ol>
  </nav>
</template>

<script setup lang="ts">
import { ChevronRightIcon } from '@heroicons/vue/20/solid'

export interface BreadcrumbItem {
  name: string
  link: string
}

defineProps({
  breadcrumbs: {
    type: Array<BreadcrumbItem>,
    required: true,
    default: () => [],
  },
})
</script>
