import { createRouter, createWebHistory } from 'vue-router'

import SubscribeForm from '../SubscribeForm.vue'
import LoginForm from '../LoginForm.vue'
import AdminDashboard from '../AdminDashboard.vue'
import SendNewsletterForm from '../SendNewsletterForm.vue'
import ChangePasswordForm from '../ChangePasswordForm.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: SubscribeForm },
    { path: '/login', component: LoginForm },
    { path: '/admin', component: AdminDashboard },
    { path: '/admin/newsletters', component: SendNewsletterForm },
    { path: '/admin/password', component: ChangePasswordForm },
  ],
})

export default router
