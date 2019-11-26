import Vue from 'vue'
import Router from 'vue-router'
import CreateQueuePage from '../pages/CreateQueue.vue'
import MainPage from '../pages/Main.vue'
import ProfilePage from '../pages/Ankets.vue'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'createQueue',
      component: CreateQueuePage
    },
    {
      path: '/main',
      name: 'main',
      component: MainPage
    },
    {
      path: '/profile',
      name: 'profile',
      component: ProfilePage
    },
    {
      path: '/block/:height',
      name: 'block',
      component: BlockPage,
      props: true
    },
    {
      path: '/transaction/:hash',
      name: 'transaction',
      component: TransactionPage,
      props: true
    }
  ]
})
