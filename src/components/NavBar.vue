<template>
  <div id="nav" class="nav">
    <router-link to="/">Home</router-link>
    <nav>
      <router-link :to="{ name: 'articles' }">Articles</router-link>
    </nav>
  </div>
</template>

<script>
export default {}
</script>

<style>

</style>
