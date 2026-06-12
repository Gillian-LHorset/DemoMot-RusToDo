<template>
  <div class="home">
    <h1>Hello world !</h1>
    <ul>
      <li v-for="todo in todos" :key="todo.id">{{ todo.todo_text }}</li>
    </ul>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import api from "../api";

const todos = ref([]);

onMounted(async () => {
  try {
    const response = await api.get("/todos");
    todos.value = response.data;
  } catch (error) {
    console.error("Erreur lors de la récupération :", error);
  }
});
</script>

<style scoped></style>
