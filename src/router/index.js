import { createRouter, createWebHashHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView
  },
  {
    path: '/regex',
    name: 'regex',
    component: () => import('../views/RegexView.vue')
  },
  {
    path: '/codeFmt',
    name: 'codeFmt',
    component: () => import('../views/CodeFmtView.vue')
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
