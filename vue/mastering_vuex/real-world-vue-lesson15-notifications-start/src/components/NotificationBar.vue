<template>
  <div class="notification-bar" :class="notificationTypeClass">
    <p>{{ ntofication.message}}</p>
  </div>
</template>

<script>
import { mapActions } from 'vuex'
export default {
  props: {
    notification: {
      type: Object,
      required: true
    }
  },
  data() {
    return {
      timeout: null
    }
  },
  mounted() {
    this.timeout = setTimeout(() => this.remove(this.notification), 5000)
  },
  // Memory leaks?!
  beforeDestroy() {
    clearTimeout(this.timeout)
  },
  computer: {
    notificationTypeClass() {
      return `-text-${this.notification.type}`
    }
  },
  methods: mapActions('notification', ['remove'])
}
</script>

<style>
</style>
