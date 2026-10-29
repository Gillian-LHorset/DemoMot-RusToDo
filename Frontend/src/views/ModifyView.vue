<template>
  <main class="page">
    <header class="header">
      <RouterLink to="/" class="back" title="Retour">←</RouterLink>
      <h1>Modifier votre todo</h1>
    </header>

    <form method="PATCH" @submit.prevent="patchTodo" class="edit_card">
      <label for="todo-text">Texte du todo</label>
      <input
        type="text"
        id="todo-text"
        v-model="newTodoText"
        placeholder="Que faut-il faire ?"
      />
      <div class="actions">
        <RouterLink to="/" class="btn ghost">Annuler</RouterLink>
        <button type="submit" class="btn primary">Modifier</button>
      </div>
    </form>
  </main>
</template>

<style scoped>
.page {
  max-width: 640px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
  font-family: system-ui, sans-serif;
}

.header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.back {
  display: grid;
  place-items: center;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: var(--beige);
  color: #374151;
  text-decoration: none;
  font-size: 1.1rem;
  transition: background 0.15s;
}

.back:hover {
  background: var(--jaune);
}

h1 {
  margin: 0;
  font-size: 1.6rem;
  letter-spacing: -0.02em;
}

.edit_card {
  padding: 1.5rem;
  border: 1px solid var(--beige);
  box-shadow: 0 1px 3px rgba(250, 80, 15, 0.08);
  border: 1px solid #e5e7eb;
  border-radius: 12px;
}

label {
  display: block;
  font-size: 0.85rem;
  font-weight: 600;
  color: #374151;
  margin-bottom: 0.5rem;
}

input {
  width: 100%;
  box-sizing: border-box;
  padding: 0.65rem 0.8rem;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  font-size: 0.95rem;
  outline: none;
  transition:
    border-color 0.15s,
    box-shadow 0.15s;
}

input:focus {
  border-color: var(--orange);
  box-shadow: 0 0 0 3px rgba(255, 130, 5, 0.2);
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  margin-top: 1.25rem;
}

.btn {
  padding: 0.6rem 1.1rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.95rem;
  cursor: pointer;
  text-decoration: none;
  transition:
    background 0.15s,
    border-color 0.15s;
}

.primary {
  border: none;
  background: var(--orange-vif);
  color: #fff;
}

.primary:hover {
  background: var(--orange);
}

.ghost {
  border: 1px solid #d1d5db;
  background: #fff;
  color: #374151;
}

.ghost:hover {
  background: var(--beige);
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
