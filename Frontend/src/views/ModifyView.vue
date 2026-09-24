<template>
  <h1>Modifier votre todo</h1>

  <form method="PATCH" @submit.prevent="patchTodo">
    <label for="todo-text">Texte du todo</label>
    <input type="text" id="todo-text" v-model="newTodoText" />
    <button type="submit">Modifier le todo</button>
  </form>
</template>

<style lang="css" scoped>
h1 {
  display: flex;
  justify-content: center;
}
form {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 5px;
  width: 100%;
  max-width: 200px;
  margin: 0 auto;
}
form > input,
form > button {
  width: 100%;
  border: 1px solid black;
  box-sizing: border-box;
}

form > button {
  cursor: pointer;
}
</style>

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
