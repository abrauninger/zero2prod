import { createRouter, createWebHistory } from 'vue-router'

import SubscribeForm from '../SubscribeForm.vue'
import LoginForm from '../LoginForm.vue'
import SendNewsletterForm from '../SendNewsletterForm.vue'
import ChangePasswordForm from '../ChangePasswordForm.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: SubscribeForm },
    { path: '/login', component: LoginForm, meta: { breadcrumb: 'Log in' } },
    {
      path: '/admin/newsletters',
      component: SendNewsletterForm,
      meta: { breadcrumb: 'Send newsletter' },
    },
    {
      path: '/admin/password',
      component: ChangePasswordForm,
      meta: { breadcrumb: 'Change password' },
    },
  ],
})

export default router
