<template>
  <div class="flex space-x-4 justify-end px-2 py-2">
    <AppBreadcrumb :breadcrumbs="generatedBreadcrumbs()" />
    <div v-if="username">
      <span class="text-gray-500">Logged in as </span>

      <Menu as="div" class="relative inline-block" v-slot="{ open }">
        <MenuButton :class="[open ? 'bg-gray-400' : '', 'flex rounded-md px-1 py-1 font-bold']">
          {{ username }}
          <ChevronDownIcon class="h-5 w-5" aria-hidden="true"
        /></MenuButton>
        <div>
          <MenuItems
            class="absolute right-0 w-56 origin-top-right bg-white rounded-md px-1 py-1 shadow-lg ring-1 ring-black/5 focus:outline-none"
          >
            <MenuItem v-slot="{ active }" class="block">
              <a
                @click="router.push('/')"
                :class="[
                  active ? 'bg-blue-500 text-white' : 'text-gray-900',
                  'cursor-default px-2 py-2 text-md rounded-md',
                ]"
                >Subscribe to newsletter</a
              >
            </MenuItem>
            <MenuItem v-slot="{ active }" class="block">
              <a
                @click="router.push('/admin/newsletters')"
                :class="[
                  active ? 'bg-blue-500 text-white' : 'text-gray-900',
                  'cursor-default px-2 py-2 text-md rounded-md',
                ]"
                >Send a newsletter issue</a
              >
            </MenuItem>
            <MenuItem v-slot="{ active }" class="block">
              <a
                @click="router.push('/admin/password')"
                :class="[
                  active ? 'bg-blue-500 text-white' : 'text-gray-900',
                  'cursor-default px-2 py-2 text-md rounded-md',
                ]"
                >Change password</a
              >
            </MenuItem>
            <MenuItem v-slot="{ active }" class="block"
              ><a
                @click="logout(router)"
                :class="[
                  active ? 'bg-blue-500 text-white' : 'text-gray-900',
                  'cursor-default px-2 py-2 text-md rounded-md',
                ]"
                >Log out</a
              ></MenuItem
            >
          </MenuItems>
        </div>
      </Menu>
    </div>
    <div v-else>
      <a
        @click="selfRequestLogin"
        class="text-gray-900 hover:bg-gray-400 rounded-md px-2 py-2 cursor-default"
        >Log in</a
      >
    </div>
  </div>
  <div>
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import { watchEffect } from 'vue'

import { RouterView, useRoute, useRouter } from 'vue-router'

import { Menu, MenuButton, MenuItems, MenuItem } from '@headlessui/vue'
import { ChevronDownIcon } from '@heroicons/vue/20/solid'

import AppBreadcrumb, { BreadcrumbItem } from './AppBreadcrumb.vue'

import { username, fetchUsername, logout, setLoginSource } from './state.ts'

const router = useRouter()
const route = useRoute()

watchEffect(async () => {
  fetchUsername()
})

// TODO: Rename?
const selfRequestLogin = async () => {
  setLoginSource(route.path)
  router.push('/login')
}

// Explanation of why we use direct calls to 'router.push' instead of RouterLink:
// https://stackoverflow.com/a/76857856

const generatedBreadcrumbs = () => {
  const breadcrumbs: BreadcrumbItem[] = []
  breadcrumbs.push({
    name: 'Home',
    link: '/',
  })

  // TODO: Simplify; right now this only enumerates one item.
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
