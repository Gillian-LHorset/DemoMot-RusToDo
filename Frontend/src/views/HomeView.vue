<template>
  <h1>Hello world !</h1>
  <div class="list">
    <ul>
      <li v-for="todo in todos" :key="todo.todo_id">{{ todo.todo_text }}</li>
    </ul>
  </div>
  <div class="list">
    <form method="POST" @submit.prevent="addTodo">
      <label for="">text</label>
      <input type="text" id="todo-text" v-model="newTodoText" />
      <button type="submit">Créer le todo</button>
    </form>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import Service from "@/services/service.js";
import api from "../api";

const todos = ref([]);
const newTodoText = ref("");

onMounted(async () => {
  try {
    Service.getTodos()
      .then((response) => (todos.value = response.data))
      .catch((error) => console.log(error));
  } catch (error) {
    console.error("Erreur lors de la récupération :", error);
  }
});

const addTodo = async () => {
  const newTodo = {
    todo_text: newTodoText.value,
  };
  console.log(newTodo);

  try {
    const response = await Service.addTodo(newTodo).then();
    console.log(response.data);
    todos.value.push(response.data);

    newTodoText.value = "";
  } catch (error) {
    console.log(error);
  }
};
</script>

<style scoped></style>
