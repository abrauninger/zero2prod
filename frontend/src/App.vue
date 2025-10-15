<template>
  <nav class="relative bg-gray-200 p-2">
    <div class="flex space-x-4">
      <RouterLink
        to="/"
        class="rounded-md px-3 py-2 text-sm font-medium bg-gray-300 text-black hover:bg-gray-400 hover:text-blue-800"
        >Subscribe</RouterLink
      >

      <div v-if="username" class="inline-block">
        <span>Logged in as </span>

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
        <a @click="selfRequestLogin" class="text-blue-800 hover:text-blue-950 cursor-default"
          >Log in</a
        >
      </div>
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
import { ChevronDownIcon } from '@heroicons/vue/20/solid'

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
</script>
