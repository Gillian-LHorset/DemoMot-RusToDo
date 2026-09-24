<template>
  <h1>Vos todos !</h1>
  <div class="list">
    <ul>
      <li v-for="todo in todos" :key="todo.todo_id">
        <div class="todo_list">
          <p>{{ todo.todo_text }}</p>

          <div class="todo_options">
            <RouterLink :to="`/modify/${todo.todo_id}`">
              <img src="../assets/icons/editIcon.svg" />
            </RouterLink>
            <button @click="deleteTodo(todo.todo_id)">
              <img src="../assets/icons/deleteIcon.svg" />
            </button>
          </div>
        </div>
      </li>
    </ul>
  </div>
  <div>
    <form method="POST" @submit.prevent="addTodo" class="todo_add">
      <label for="todo-text">Ajouter un todo</label>
      <input type="text" id="todo-text" v-model="newTodoText" />
      <button type="submit">Créer le todo</button>
    </form>
  </div>
</template>

<style lang="css" scoped>
h1 {
  display: flex;
  justify-content: center;
}

/* todo list */
.list {
  display: flex;
  justify-content: center;
}

.list > ul {
  list-style: none;
  margin: 0;
  padding: 0;
  width: 100%;
}
.list > ul > li {
  width: 100%;
  margin-bottom: 20px;
  display: flex;
  justify-content: center;
}

.todo_list {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin: 0 auto;
  width: 100%;
  padding: 2px 10px;
  border: solid 1px;
  border-radius: 25px;
  min-width: 300px;
  max-width: 800px;
}

.todo_options {
  display: flex;
  flex-direction: row;
  justify-content: end;
  align-items: center;
}

.todo_options > button {
  all: unset;
  cursor: pointer;
}

.todo_options > * > img {
  width: 21px;
}

/* add todo */

.todo_add {
  position: fixed;
  top: 100px;
  left: 20px;
  display: flex;
  flex-direction: column;
  width: 200px;
  gap: 5px;
}

.todo_add > input {
}
</style>

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
</script>

<style scoped></style>
