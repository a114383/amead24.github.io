<template>
  <div id="nav" class="nav">
    <router-link to="/">Home</router-link>
    <nav>
      <router-link :to="{ name: 'article-list' }">Articles</router-link>
    </nav>
  </div>
</template>

<script>
export default {}
</script>

<style>

</style>
