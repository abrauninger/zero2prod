<template>
  <nav>
    <div class="flex space-x-4 justify-between px-2 py-2 bg-gray-200">
      <AppBreadcrumb :breadcrumbs="generatedBreadcrumbs()" />
      <div v-if="username">
        <span class="text-gray-500">Logged in as </span>

        <Menu as="div" class="relative inline-block" v-slot="{ open }">
          <MenuButton
            :class="[
              open ? 'bg-gray-400' : '',
              'flex rounded-md px-1 py-1 font-bold hover:bg-gray-400',
            ]"
          >
            {{ username }}
            <ChevronDownIcon class="h-5 w-5" aria-hidden="true"
          /></MenuButton>
          <div>
            <MenuItems
              class="absolute right-0 w-56 origin-top-right bg-white rounded-md px-1 py-1 shadow-lg ring-1 ring-black/5 focus:outline-none"
            >
              <MenuItem
                v-for="command in loggedInMenuCommands"
                v-slot="{ active }"
                :key="command.route"
                class="block"
              >
                <a
                  :class="[
                    active ? 'bg-blue-500 text-white' : 'text-gray-900',
                    'cursor-default px-2 py-2 text-md rounded-md',
                  ]"
                  @click="command.click ? command.click(router) : router.push(command.route)"
                  >{{ command.label }}</a
                >
              </MenuItem>
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
  </nav>
  <div>
    <RouterView />
  </div>
</template>

<script setup lang="ts">
import { computed, watchEffect } from 'vue'

import { RouterView, useRoute, useRouter } from 'vue-router'
import type { Router } from 'vue-router'

import { Menu, MenuButton, MenuItems, MenuItem } from '@headlessui/vue'
import { ChevronDownIcon } from '@heroicons/vue/20/solid'

import AppBreadcrumb from './AppBreadcrumb.vue'
import type { BreadcrumbItem } from './AppBreadcrumb.vue'

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

const loggedInMenuCommands = computed(() => {
  if (username.value) {
    return [
      { label: 'Subscribe to newsletter', route: '/' },
      { label: 'Send a newsletter issue', route: '/admin/newsletters' },
      { label: 'Change password', route: '/admin/password' },
      { label: 'Log out', click: (router: Router) => logout(router) },
    ]
  } else {
    return []
  }
})
</script>
