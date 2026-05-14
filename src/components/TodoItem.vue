<script setup lang="ts">
import type { Todo } from "../types/todo";

defineProps<{
  todo: Todo;
}>();

const emit = defineEmits<{
  toggle: [todo: Todo];
  remove: [todo: Todo];
}>();

const dueTagLabel: Record<string, string> = {
  today: "今日内",
  within5Hours: "5h内",
  threeDays: "三日内",
  thisWeek: "本周",
  thisMonth: "本月",
  longTerm: "长期计划"
};

const priorityTagLabel: Record<string, string> = {
  high: "高优先",
  medium: "中优先",
  low: "低优先"
};
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
      <div class="todo-item__content">
        <span class="todo-item__title">{{ todo.title }}</span>
        <div class="todo-item__tags">
          <span v-if="todo.dueTag" class="todo-item__tag">{{ dueTagLabel[todo.dueTag] }}</span>
          <span v-if="todo.priorityTag" class="todo-item__tag todo-item__tag--priority">{{ priorityTagLabel[todo.priorityTag] }}</span>
        </div>
      </div>
    </label>
    <button class="todo-item__delete" type="button" @click="emit('remove', todo)">删除</button>
  </li>
</template>
