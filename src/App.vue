<template>
  <v-app>

    <v-app-bar app color="red" dark>
      <v-toolbar-title>
        Tony Mead - Software & Economics
      </v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn to="/" text rounded>Home</v-btn>
      <v-btn to="/articles" text rounded>Articles</v-btn>
    </v-app-bar>

    <v-content>
      <router-view></router-view>
    </v-content>

    <v-footer color="red" dark app>
    </v-footer>

  </v-app>
</template>

<script>
export default {
  name: 'App',
  data: () => ({
    //
  })
}
</script>
