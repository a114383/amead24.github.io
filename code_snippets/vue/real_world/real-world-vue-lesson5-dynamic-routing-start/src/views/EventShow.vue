<template>
  <!-- Conversely you can access the route object directly if you don't pass in props -->
  <!-- <h1>Showing Event #{{ $route.params.id }}</h1> -->
  <h1>Showing Event #{{ id }}</h1>
</template>

<script>
export default {
  props: ['id']
}
</script>