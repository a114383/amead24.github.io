<template>
  <div>
    <h1>Events Listing</h1>
    <!-- 
      Most interestingly we're looping through each event
      and then passing those as a prop on an individual basis
    -->
    <EventCard v-for="event in events" :key="event.id" :event="event" />
  </div>
</template>

<script>
import EventCard from '@/components/EventCard.vue'
import EventService from '@/services/EventServices.js'

export default {
  data() {
    return {
      events: []
    }
  },
  components: {
    EventCard
  },
  created() {
    EventService.getEvents()
      .then(response => {
        this.events = response.data
      })
      .catch(errors => {
        console.log(errors)
      })
  }
}
</script>
