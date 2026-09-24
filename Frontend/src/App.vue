<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
  <header>
    <nav>
      <RouterLink to="/">home</RouterLink>
      <RouterLink to="/">pas home</RouterLink>
      <RouterLink to="/">enfaite home</RouterLink>
    </nav>
  </header>

  <RouterView />
</template>

<style scoped>
header {
  position: sticky;
  top: 0;
  left: 0;
  margin: 0;
  padding: 30px;
  display: flex;
  flex-direction: row;
  justify-content: start;
  background-color: lightgray;
}

nav {
  margin: 0;
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
  align-items: center;
  gap: 50px;
}
</style>
