import axios from 'axios'
import NProgress from 'nprogress'

const apiClient = axios.create({
  baseURL: `http://localhost:3000`,
  withCredentials: false, // This is the default
  headers: {
    Accept: 'application/json',
    'Content-Type': 'application/json'
  }
})

/*
 * This solution doesn't scale if multiple requests were pending,
 * and so you'd want to centralize that counter.  One suggestion surrounds
 * creating a vuex state module that gets incremented and decremented with 
 * every api call, with computed properties that trigger .start() & .done()
 */
    

/* 
 * Reasons for interceptors:
 * 1. on request to set authorization tokens
 * 2. on response to format or filter data
 * 3. on response catch 401 not authorized
 */

apiClient.interceptors.request.use(config => {
  NProgress.start()
  return config
})

apiClient.interceptors.response.use(response => {
  NProgress.done()
  return response
})


export default {
  getEvents(perPage, page) {
    return apiClient.get('/events?_limit=' + perPage + '&_page=' + page)
  },
  getEvent(id) {
    return apiClient.get('/events/' + id)
  },
  postEvent(event) {
    return apiClient.post('/events', event)
  }
}
