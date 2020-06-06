import EventService from '@/services/EventService.js'

// See NOTE on 46 for further clarification
// accessing namespaced modules with dispatch('<namespace>/function`)
export const namespaced = true

/* 
Conversely you can keep the module object structure the same
as the other .vue/.js code by `export default {...}`.

const allows for external functions? Not sure what that means...

*/
export const state = {
  events: [],
  eventsTotal: 0,
  event: {}
}

export const mutations = {
  ADD_EVENT(state, event) {
    state.events.push(event)
  },
  SET_EVENTS(state, events) {
    state.events = events
  },
  SET_EVENTS_TOTAL(state, eventsTotal) {
    state.eventsTotal = eventsTotal
  },
  SET_EVENT(state, event) {
    state.event = event
  }
}

export const actions = {
  // You can't pass in { commit, state } because when in modules
  // the `state` context object only gives you access to the local
  // state values - instead you need rootState to get base state
  createEvent({ commit, rootState }, event) {
    console.log(`User ${rootState.user.user.name} creating event.`)
    // we can also modify other modules state using the same action/mutation
    // dispatch functionality that we are use to with components modifing vuex state
    // dispatch('actionToCall')
    // -----------------------------------------------------------------------------
    // Unless modules are namespaced
    // NOTE: All mutations, actions, and getters are globally namespaced
    // this is to allow multiple functions to get triggered by a single event
    // the example given: dispatch('purchaseItem') can force a cart or user module
    // to get updated at the same time a logging module can log out the event
    return EventService.postEvent(event).then(() => {
      commit('ADD_EVENT', event)
    })
  },
  fetchEvents({ commit }, { perPage, page }) {
    EventService.getEvents(perPage, page)
      .then(response => {
        commit('SET_EVENTS_TOTAL', parseInt(response.headers['x-total-count']))
        commit('SET_EVENTS', response.data)
      })
      .catch(error => {
        console.log('There was an error:', error.response)
      })
  },
  fetchEvent({ commit, getters }, id) {
    var event = getters.getEventById(id)

    if (event) {
      commit('SET_EVENT', event)
    } else {
      EventService.getEvent(id)
        .then(response => {
          commit('SET_EVENT', response.data)
        })
        .catch(error => {
          console.log('There was an error:', error.response)
        })
    }
  }
}

export const getters = {
  getEventById: state => id => {
    return state.events.find(event => event.id === id)
  }
}
