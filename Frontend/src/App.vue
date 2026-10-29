<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
  <div class="app">
    <header class="topbar">
      <nav class="nav">
        <RouterLink to="/" class="brand">
          <img src="./assets/icons/logo.svg" alt="" class="logo" v-if="false" />
          Rus<span class="accent">ToDo</span>
        </RouterLink>
      </nav>
    </header>

    <main class="content">
      <RouterView />
    </main>
  </div>
</template>

<style>
body {
  margin: 0;
  background: #f3f4f6;
  color: #111827;
  -webkit-font-smoothing: antialiased;
}

.topbar {
  position: sticky;
  top: 0;
  z-index: 10;
  background: rgba(255, 250, 235, 0.85);
  backdrop-filter: blur(8px);
  border-bottom: 1px solid var(--beige);
}

.nav {
  max-width: 640px;
  margin: 0 auto;
  padding: 0.75rem 1.5rem;
}

.brand {
  font-size: 1.15rem;
  font-weight: 700;
  letter-spacing: -0.02em;
  text-decoration: none;
  color: #111827;
}

.accent {
  color: var(--orange-vif);
}

.brand.router-link-active {
  color: #111827;
}

.content {
  min-height: calc(100vh - 57px);
}
</style>
