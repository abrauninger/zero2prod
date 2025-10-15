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

        <Menu as="div" class="relative inline-block">
          <MenuButton class="inline-flex">
            <strong>{{ username }}</strong
            ><ChevronDownIcon class="h-5 w-5" aria-hidden="true"
          /></MenuButton>
          <div>
            <MenuItems class="absolute right-0 w-52 origin-top-right bg-white rounded-md px-3 py-1">
              <MenuItem class="block">
                <a class="cursor-default" @click="routerLinkNavigate('/admin/newsletters')"
                  >Send a newsletter issue</a
                >
              </MenuItem>
              <MenuItem class="block">
                <a class="cursor-default" @click="routerLinkNavigate('/admin/password')"
                  >Change password</a
                >
              </MenuItem>
              <MenuItem class="block"
                ><a @click="logout(router)" class="cursor-default">Log out</a></MenuItem
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

// https://stackoverflow.com/a/76857856
const routerLinkNavigate = (href: string) => {
  router.push(href)
}
</script>
