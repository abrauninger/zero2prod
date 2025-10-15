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
      <div v-if="username" class="inline-block">
        <span
          >Logged in as <strong>{{ username }}</strong></span
        >
        <AppButton @click="logout">Log out</AppButton>
      </div>
      <div v-else>
        <AppButton @click="selfRequestLogin">Log in</AppButton>
      </div>
      <Menu>
        <MenuButton>Options</MenuButton>
        <MenuItems>
          <MenuItem><a>Foo</a></MenuItem>
          <MenuItem><a>Bar</a></MenuItem>
        </MenuItems>
      </Menu>
    </div>
  </nav>
  <div>
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import { watchEffect } from 'vue'

import { RouterLink, RouterView, useRoute, useRouter } from 'vue-router'

import { Menu, MenuButton, MenuItems, MenuItem } from '@headlessui/vue'

import { username, fetchUsername, logout, setLoginSource } from './state.ts'

import AppButton from './AppButton.vue'

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
</script>
