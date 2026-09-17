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
import { ref, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";

const router = useRouter();

const route = useRoute();
const todoId = route.params.id;
const newTodoText = ref("");

const todo = ref();
onMounted(async () => {
  try {
    const response = await Service.getTodo(todoId);
    todo.value = response.data;
    newTodoText.value = response.data.todo_text;
  } catch (error) {
    console.error("Erreur lors de la récupération :", error);
  }
});

const patchTodo = async () => {
  const newTodo = {
    todo_text: newTodoText.value,
  };

  try {
    const response = await Service.modifyTodo(todoId, newTodo).then();
    router.push("/");
  } catch (error) {
    console.log(error);
  }
};
</script>
