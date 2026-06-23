<template>
  <h1>Hello world !</h1>
  <div class="list">
    <ul>
      <li v-for="todo in todos" :key="todo.todo_id">
        <div class="todo_list">
          <p>{{ todo.todo_text }}</p>

          <RouterLink :href="/modify/ + todo.todo_id">Modifier</RouterLink>
          <button @click="deleteTodo(todo.todo_id)">supprimer</button>
        </div>
      </li>
    </ul>
  </div>
  <div class="list">
    <form method="POST" @submit.prevent="addTodo">
      <label for="todo-text">text</label>
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

  try {
    const response = await Service.addTodo(newTodo).then();

    todos.value.push(response.data);

    newTodoText.value = "";
  } catch (error) {
    console.log(error);
  }
};

function deleteTodo(todo_id) {
  Service.deleteTodo(todo_id);

  window.location.reload();
}

function modifyTodo(todo_id) {}
</script>

<style scoped>
.todo_list {
  display: flex;
  flex-direction: row;
  gap: 30px;
  margin-bottom: 20px;
}
</style>
