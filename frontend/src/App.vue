<template>
  <nav>
    <div class="flex space-x-4 justify-between px-2 py-2 bg-gray-200">
      <AppBreadcrumb :breadcrumbs="generatedBreadcrumbs()" />
      <UserMenu />
    </div>
  </nav>
  <div>
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import { watchEffect } from 'vue'

import { RouterView, useRoute } from 'vue-router'

import AppBreadcrumb from './AppBreadcrumb.vue'
import type { BreadcrumbItem } from './AppBreadcrumb.vue'
import UserMenu from './UserMenu.vue'

import { fetchUsername } from './state.ts'

const route = useRoute()

watchEffect(async () => {
  fetchUsername()
})

const generatedBreadcrumbs = () => {
  const breadcrumbs: BreadcrumbItem[] = []

  console.log(route.matched)

  if (
    route.matched.length > 0 &&
    route.matched[0] &&
    route.matched[0].meta &&
    !route.matched[0].meta.breadcrumb
  ) {
    breadcrumbs.push({
      name: 'Home',
    })
  } else {
    breadcrumbs.push({
      name: 'Home',
      link: '/',
    })
  }

  // TODO: Simplify; right now this only enumerates at most one item.
  route.matched.forEach((route) => {
    if (route.meta && route.meta.breadcrumb) {
      breadcrumbs.push({
        name: route.meta.breadcrumb,
      })
    }
  })
  console.log(breadcrumbs)
  return breadcrumbs
}
</script>
