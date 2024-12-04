import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [{ path: '', component: () => import('pages/IndexPage.vue') }],
  },
  {
    path: '/auth',
    component: () => import('layouts/Guest/GuestLayout.vue'),
    children: [
      {
        path: 'login',
        component: () => import('pages/Login/LoginPage.vue'),
        name: 'login',
      },
      {
        path: 'register',
        component: () => import('pages/Register/RegisterPage.vue'),
        name: 'register',
      },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
