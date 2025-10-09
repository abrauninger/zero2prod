import { createRouter, createWebHistory } from 'vue-router'

import SubscribeForm from '../SubscribeForm.vue'
import LoginForm from '../LoginForm.vue'
import AdminDashboard from '../AdminDashboard.vue'
import SendNewsLetterForm from '../SendNewsletter.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: SubscribeForm },
    { path: '/login', component: LoginForm },
    { path: '/admin', component: AdminDashboard },
    { path: '/admin/newsletters', component: SendNewsLetterForm },
  ],
})

export default router
