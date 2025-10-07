import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    // component: HomeView
    component: () => import('../views/index.vue')
  },
  // {
  //   path: '/regex',
  //   name: 'regex',
  //   component: () => import('../views/regex.vue')
  // },
  // {
  //   path: '/codeFmt',
  //   name: 'codeFmt',
  //   component: () => import('../views/codeFmt.vue')
  // }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
