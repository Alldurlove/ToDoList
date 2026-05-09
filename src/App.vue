<script setup lang="ts">
import { computed, onMounted } from "vue";
import TodoInput from "./components/TodoInput.vue";
import TodoList from "./components/TodoList.vue";
import { useTodoStore } from "./stores/todoStore";

const store = useTodoStore();
const pendingItems = computed(() => store.pendingItems.value);
const completedItems = computed(() => store.completedItems.value);

onMounted(() => {
  void store.refresh();
});
</script>

<template>
  <main class="app">
    <section class="panel">
      <header class="panel__header">
        <h1 class="panel__title">ToDoList</h1>
        <p class="panel__subtitle">轻量、简约、专注完成</p>
      </header>

      <TodoInput @submit="store.addTodo" />

      <p v-if="store.error" class="status status--error">{{ store.error }}</p>
      <p v-else-if="store.loading" class="status">加载中...</p>

      <section class="todo-group">
        <h2 class="todo-group__title">待完成</h2>
        <TodoList :items="pendingItems" empty-text="太棒了，当前没有待办" @toggle="store.toggleTodo" @remove="store.removeTodo" />
      </section>

      <section class="todo-group">
        <h2 class="todo-group__title">已完成</h2>
        <TodoList :items="completedItems" empty-text="还没有完成的任务" @toggle="store.toggleTodo" @remove="store.removeTodo" />
      </section>
    </section>
  </main>
</template>
