<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import type { Todo } from "../types/todo";

const props = defineProps<{
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

const now = ref(Date.now());
let timer: number | null = null;

function formatRemaining(dueAt?: string) {
  if (!dueAt) {
    return null;
  }

  const diffMs = new Date(dueAt).getTime() - now.value;
  if (diffMs <= 0) {
    return "已到期";
  }

  const hours = Math.ceil(diffMs / (1000 * 60 * 60));
  const days = Math.ceil(diffMs / (1000 * 60 * 60 * 24));

  if (hours <= 24) {
    return `剩余 ${hours}h`;
  }

  if (days <= 30) {
    return `剩余 ${days}日`;
  }

  return "一个月以上";
}

const remainingText = computed(() => formatRemaining(props.todo.dueAt));

onMounted(() => {
  timer = window.setInterval(() => {
    now.value = Date.now();
  }, 60_000);
});

onUnmounted(() => {
  if (timer !== null) {
    window.clearInterval(timer);
  }
});
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
          <span v-if="remainingText" class="todo-item__tag todo-item__tag--time">{{ remainingText }}</span>
        </div>
      </div>
    </label>
    <button class="todo-item__delete" type="button" @click="emit('remove', todo)">删除</button>
  </li>
</template>
