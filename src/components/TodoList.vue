<script setup lang="ts">
import TodoItem from "./TodoItem.vue";
import type { Todo } from "../types/todo";

defineProps<{
  items: Todo[];
  emptyText?: string;
}>();

const emit = defineEmits<{
  toggle: [todo: Todo];
  remove: [todo: Todo];
}>();
</script>

<template>
  <ul v-if="items.length" class="todo-list">
    <TodoItem
      v-for="item in items"
      :key="item.id"
      :todo="item"
      @toggle="emit('toggle', $event)"
      @remove="emit('remove', $event)"
    />
  </ul>
  <p v-else class="todo-list__empty">{{ emptyText ?? "暂无任务" }}</p>
</template>
