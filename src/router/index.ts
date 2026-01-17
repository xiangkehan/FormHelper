import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/views/HomeView.vue')
  },
  {
    path: '/persons',
    name: 'persons',
    component: () => import('@/views/PersonsView.vue')
  },
  {
    path: '/files',
    name: 'files',
    component: () => import('@/views/FilesView.vue')
  },
  {
    path: '/data',
    name: 'data',
    component: () => import('@/views/DataView.vue')
  },
  {
    path: '/settings',
    name: 'settings',
    component: () => import('@/views/SettingsView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
