<template>
  <main class="page">
    <header class="header">
      <h1>Vos todos !</h1>
    </header>

    <div class="list">
      <ul v-if="todos.length">
        <TransitionGroup name="fade">
          <li v-for="todo in todos" :key="todo.todo_id" class="todo">
            <p>{{ todo.todo_text }}</p>

            <div class="todo_options">
              <RouterLink
                :to="`/modify/${todo.todo_id}`"
                class="icon-btn"
                title="Modifier"
              >
                <img src="../assets/icons/editIcon.svg" alt="Modifier" />
              </RouterLink>
              <button
                @click="deleteTodo(todo.todo_id)"
                class="icon-btn danger"
                title="Supprimer"
              >
                <img src="../assets/icons/deleteIcon.svg" alt="Supprimer" />
              </button>
            </div>
          </li>
        </TransitionGroup>
      </ul>
      <p v-else class="empty">Aucun todo pour le moment 🎉</p>
    </div>

    <form method="POST" @submit.prevent="addTodo" class="todo_add">
      <label for="todo-text">Ajouter un todo</label>
      <div class="add_row">
        <input
          type="text"
          id="todo-text"
          v-model="newTodoText"
          placeholder="Que faut-il faire ?"
        />
        <button type="submit">Créer</button>
      </div>
    </form>
  </main>
</template>

<style lang="css" scoped>
.page {
  max-width: 640px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
  font-family: system-ui, sans-serif;
}

.header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 1.5rem;
}

h1 {
  margin: 0;
  font-size: 2rem;
  letter-spacing: -0.02em;
}

.counter {
  font-size: 0.9rem;
  color: var(--violet);
}

.list ul {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.todo {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.9rem 1.1rem;
  background: #fff;
  border: 1px solid var(--beige);
  box-shadow: 0 1px 3px rgba(250, 80, 15, 0.08);
  border-radius: 12px;
  transition:
    transform 0.15s,
    box-shadow 0.15s;
}

.todo:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.todo p {
  margin: 0;
  word-break: break-word;
}

.todo_options {
  display: flex;
  gap: 0.4rem;
  flex-shrink: 0;
}

.icon-btn {
  display: grid;
  place-items: center;
  width: 34px;
  height: 34px;
  border: none;
  border-radius: 8px;
  background: var(--beige);
  cursor: pointer;
  transition: background 0.15s;
}

.icon-btn:hover {
  background: var(--jaune);
}
.danger:hover {
  background: #ffd9c9;
}
.icon-btn img {
  width: 16px;
  height: 16px;
}

.todo_add {
  margin-top: 2rem;
  padding: 1.25rem;
  background: var(--beige);
  border: 1px solid var(--jaune);
  border-radius: 12px;
}

.todo_add label {
  display: block;
  font-size: 0.85rem;
  font-weight: 600;
  color: #374151;
  margin-bottom: 0.5rem;
}

.add_row {
  display: flex;
  gap: 0.5rem;
}

.add_row input {
  flex: 1;
  padding: 0.6rem 0.8rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.95rem;
  outline: none;
  transition:
    border-color 0.15s,
    box-shadow 0.15s;
}

.add_row input:focus {
  border-color: var(--orange);
  box-shadow: 0 0 0 3px rgba(255, 130, 5, 0.2);
}

.add_row button {
  padding: 0.6rem 1.1rem;
  border: none;
  border-radius: 8px;
  background: var(--orange-vif);
  color: #fff;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.15s;
}

.add_row button:hover {
  background: var(--orange);
}

.empty {
  text-align: center;
  color: #9ca3af;
  padding: 2rem 0;
}

.fade-enter-active,
.fade-leave-active {
  transition:
    opacity 0.2s,
    transform 0.2s;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateX(-8px);
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
