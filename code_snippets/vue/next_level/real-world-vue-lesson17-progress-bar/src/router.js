import Vue from 'vue'
import Router from 'vue-router'
import EventCreate from './views/EventCreate.vue'
import EventList from './views/EventList.vue'
import EventShow from './views/EventShow.vue'

Vue.use(Router)

/*
 * Vue Route Navigation Guard Parameters
 * Navigation Hook AKA Lifecycle hooks
 * beforeRouteEnter(routeTo, routeFrom, next)
 *    before component is created - no access to "this"
 * beforeRouteUpdate(...)
 *    called when route changes, but still using same component
 * beforeRouteLeave(...)
 *    called when component is navigated away from
 *
 * next() # confirm
 * next(false) # cancel
 * next({name: "event-list"}) # redirect
 *
 * beforeRouteLeave(routeTo, routeFrom, next) {
 *   const answer = window.confirm(
 *     'Do you really want to leave? You have unsaved changes!'
 *   )
 *   if (answer) {
 *     next() // <-- Confirms the navigation
 *   } else {
 *     next(false) // <-- Cancels the navigation
 *   }
 * },
 */

export default new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      name: 'event-list',
      component: EventList
    },
    {
      path: '/event/create',
      name: 'event-create',
      component: EventCreate
    },
    {
      path: '/event/:id',
      name: 'event-show',
      component: EventShow,
      props: true
    }
  ]
})
