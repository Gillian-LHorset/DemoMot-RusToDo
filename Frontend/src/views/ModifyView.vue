<template>
  <h1>hello</h1>

  <form method="PATCH" @submit.prevent="patchTodo">
    <label for="todo-text">text</label>
    <input type="text" id="todo-text" v-model="newTodoText" />
    <button type="submit">Modifier le todo</button>
  </form>
</template>
<script setup>
import Service from "@/services/service.js";
import { useRoute } from "vue-router";

const route = useRoute();
const todoId = route.params.id;

const patchTodo = async () => {
  const newTodo = {
    todo_text: newTodoText.value,
  };

  try {
    const response = await Service.modifyTodo(todoId, newTodo).then();
  } catch (error) {
    console.log(error);
  }
};
</script>
