import Vue from 'vue'
import Vuex from 'vuex'

// Still not clear on "@" vs. "./"
import EventService from '@/services/EventService.js'

Vue.use(Vuex)

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
    events: []
  },
  mutations: {
    // Sync way to modify the state
    ADD_EVENT(state, event) {
      state.events.push(event)
    }
  },
  actions: {
    // Async way to modify the state
    // Always put mutations within Actions, as app grows it helps avoid refactor
    createEvent({ commit }, event) {
      EventService.postEvent(event).then(() => {
        // because actions are async we want to wait until the api call has finished
        // before we update the events on the front end/vuex  state list
        commit('ADD_EVENT', event)
        // Then whatever trigger this event can wait as well in order
        // to make sure that this is idempotent-ish
      })
    }
  },
  getters: {
    getEventById: state => id => {
      return state.events.find(event => event.id === id)
    }
  }
})
