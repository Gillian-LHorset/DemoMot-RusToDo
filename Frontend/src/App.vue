<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
  <header>
    <nav>
      <RouterLink to="/">Nav 1</RouterLink>
      <p>|</p>
      <RouterLink to="/">Nav 2</RouterLink>
    </nav>
  </header>

  <RouterView />
</template>

<style scoped></style>
