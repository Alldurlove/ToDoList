<script setup lang="ts">
import type { Todo } from "../types/todo";

defineProps<{
  todo: Todo;
}>();

const emit = defineEmits<{
  toggle: [todo: Todo];
  remove: [todo: Todo];
}>();
</script>

<template>
  <li class="todo-item" :class="{ 'todo-item--completed': todo.completed }">
    <label class="todo-item__label">
      <input
        class="todo-item__checkbox"
        type="checkbox"
        :checked="todo.completed"
        @change="emit('toggle', todo)"
      />
      <span class="todo-item__title">{{ todo.title }}</span>
    </label>
    <button class="todo-item__delete" type="button" @click="emit('remove', todo)">删除</button>
  </li>
</template>
