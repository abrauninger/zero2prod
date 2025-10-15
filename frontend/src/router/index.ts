import { createRouter, createWebHistory } from 'vue-router'

import SubscribeForm from '../SubscribeForm.vue'
import LoginForm from '../LoginForm.vue'
import SendNewsletterForm from '../SendNewsletterForm.vue'
import ChangePasswordForm from '../ChangePasswordForm.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', component: SubscribeForm, name: 'Subscribe', meta: { breadcrumb: 'Subscribe' } },
    { path: '/login', component: LoginForm, name: 'Log in', meta: { breadcrumb: 'Log in' } },
    {
      path: '/admin/newsletters',
      component: SendNewsletterForm,
      name: 'Send newsletter',
      meta: { breadcrumb: 'Send newsletter' },
    },
    {
      path: '/admin/password',
      component: ChangePasswordForm,
      name: 'Change password',
      meta: { breadcrumb: 'Change password' },
    },
  ],
})

export default router
