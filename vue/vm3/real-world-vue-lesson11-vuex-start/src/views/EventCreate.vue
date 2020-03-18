<template>
  <div>
    <!-- 
      Accessing the store directly - but it's better to use computed properties
      <h1>Create Event {{ $store.state.user.name }}</h1> 
    -->
    <h1>Create Event {{ userName }}</h1>
    <p>{{ userId }}</p>
    <!-- <p>Calling state getter with parameter: {{ getEvent(1) }}</p> -->
    <p>Calling state getter with parameter: {{ getEventById(1) }}</p>
    <ul>
      <li v-for="cat in categories" :key="cat">{{ cat }}</li>
    </ul>
  </div>
</template>

<script>
import { mapState, mapGetters } from 'vuex'

export default {
  // Then we only need to put data that needs be shared inside
  // the store, otherwise it's okay to go inside the component.data?
  // ----------------------------------------------- //
  // Why are we not importing the store?
  // I think because App.vue loads this in as a child
  // and thus inherits all of the parent variables?
  computed: {
    // it's better to save getters that are used across multiple
    // components in the store.js than on each component
    catLength() {
      //   return this.$store.categories.length
      return this.$store.getters.catLength
    },
    // getEvent() {
    //   return this.$store.getters.getEventById
    // },
    ...mapGetters(['getEventById']),
    ...mapState({
      // before we used mapState we could access like this
      // userName() { return this.$store.state.user.name }
      // userId() { return this.$store.state.user.id }
      //
      userName: state => state.user.name,
      userId: state => state.user.id,
      // categories: state => state.user.categories
      // conversely you can directly access top level obj
      categories: 'categories'
    })
  }
}
</script>

<style></style>
