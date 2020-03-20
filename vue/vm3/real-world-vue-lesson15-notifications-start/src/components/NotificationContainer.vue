<template>
  <div class="notification-container">
    <NotificationBar
      v-for="notification in notifications"
      :key="notification.id"
      :notification="notification"
    ></NotificationBar>
  </div>
</template>

<script>
import { mapState } from 'vuex'
import NotificationBar from '@/components/NotificationBar.vue'

export default {
  components: {
    NotificationBar
  },
  computer: mapState('notification', ['notification'])
}
</script>

<style>
</style>
