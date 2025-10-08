import { createRouter, createWebHistory } from 'vue-router'

import SubscribeForm from '../SubscribeForm.vue'
import AdminDashboard from '../AdminDashboard.vue'
import LoginForm from '../LoginForm.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: SubscribeForm },
    { path: '/admin', component: AdminDashboard },
    { path: '/login', component: LoginForm },
  ],
})

export default router
