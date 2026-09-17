import axios from "axios";

const apiClient = axios.create({
  baseUrl: import.meta.env.VITE_API_URL,
  withCredentials: false,
  headers: {
    Accept: "application/json",
    "Content-Type": "application/json",
  },
});

export default {
  getTodos() {
    return apiClient.get("/api/todos");
  },
  getTodo(todoId) {
    return apiClient.get(`/api/todo/${todoId}`);
  },
  addTodo(newTodo) {
    return apiClient.post("/api/todos", newTodo);
  },
  deleteTodo(todoId) {
    return apiClient.delete(`/api/todo/${todoId}`);
  },
  modifyTodo(todoId, modifiedTodo) {
    return apiClient.put(`/api/todo/${todoId}`, modifiedTodo);
  },
};
