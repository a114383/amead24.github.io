import Vue from 'vue'
import Vuex from 'vuex'
import EventService from '@/services/EventService.js'

Vue.use(Vuex)

/*
One nice thought while doing this is that a component can
on create make some call to set the vuex store with some super long computation
time api calls (maybe graphs or data intensive stuff).  

While that's loading though the rest of your application will still be rendered.
What's nice though is that as stuff comes online a user can start interacting with
one graph, and when they go to the next page (assuming it's finished loading) there will be
no lag time.  Switching back and forth between the pages would be instant as the server
rendered everything once and then stored it client side.

Caching then becomes super easy on the server side to integrate into the front end,
you return the raw data object and the front end handles everything from there.

TODO/NOTE: Caching can be done client side with Vue, not discussed here.

*/
export default new Vuex.Store({
  state: {
    user: { id: 'abc123', name: 'Adam Jahr' },
    categories: [
      'sustainability',
      'nature',
      'animal welfare',
      'housing',
      'education',
      'food',
      'community'
    ],
    /*
      One thing i'm having a hard time understanding though is that the store
      now holds data for the EventShow, EventList, and EventCreate components.

      When I would have though only things that needed shared across components
      would be placed in the store.  Is there no reason for federating local data?
    */
    event: {},
    events: [],
    totalPages: 0
  },
  mutations: {
    ADD_EVENT(state, event) {
      state.events.push(event)
    },
    SET_EVENT(state, event) {
      state.event = event
    },
    SET_EVENTS(state, events) {
      state.events = events
    },
    SET_TOTAL_PAGES(state, totalPages) {
      state.totalPages = totalPages
    }
  },
  actions: {
    createEvent({ commit }, event) {
      return EventService.postEvent(event).then(() => {
        commit('ADD_EVENT', event)
      })
    },
    fetchEvent({ commit, getters }, id) {
      // checking if this.events already has the data
      // in which case we don't need to make a new API call
      var event = getters.getEventById(id)

      if (event) {
        commit('SET_EVENT', event)
      } else {
        EventService.getEvent(id)
          .then(response => {
            commit('SET_EVENT', response.data)
          })
          .catch(error => {
            console.log(`There was an error: ${error.response}.`)
          })
      }
    },
    // The brackets are called 'context objects',
    // I think of it as members of the object that you want, so you're just
    // passing in the commit method for this current Vuex.store() object
    // -----------------------------------------------------------------
    // Secondary payload (my parameters), in Actions or mutations are either
    // a single variable or an object (which it is in this case)
    fetchEvents({ commit }, { perPage, page }) {
      EventService.getEvents(perPage, page)
        .then(response => {
          let totalPages = Math.floor(
            response.headers['x-total-count'] / perPage
          )
          commit('SET_TOTAL_PAGES', totalPages)
          commit('SET_EVENTS', response.data)
        })
        .catch(error => {
          console.log('There was an error:', error.response)
        })
    }
  },
  getters: {
    getEventById: state => id => {
      return state.events.find(event => event.id === id)
    }
  }
})
