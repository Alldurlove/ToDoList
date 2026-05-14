<script setup lang="ts">
import { ref } from "vue";
import type { CreateTodoInput, DueTag, PriorityTag } from "../types/todo";

const emit = defineEmits<{
  submit: [input: CreateTodoInput];
}>();

const title = ref("");
const dueTag = ref<DueTag>("longTerm");
const priorityTag = ref<PriorityTag>("medium");

function onSubmit() {
  const value = title.value.trim();
  if (!value) {
    return;
  }
  emit("submit", {
    title: value,
    dueTag: dueTag.value,
    priorityTag: priorityTag.value
  });
  title.value = "";
  dueTag.value = "longTerm";
  priorityTag.value = "medium";
}
</script>

<template>
  <form class="todo-input" @submit.prevent="onSubmit">
    <input
      v-model="title"
      class="todo-input__field"
      type="text"
      placeholder="输入一个待办事项..."
      maxlength="120"
      autocomplete="off"
    />
    <select v-model="dueTag" class="todo-input__select" aria-label="截止时间">
      <option value="today">今日内</option>
      <option value="within5Hours">5h内</option>
      <option value="threeDays">三日内</option>
      <option value="thisWeek">本周</option>
      <option value="thisMonth">本月</option>
      <option value="longTerm">长期计划</option>
    </select>
    <select v-model="priorityTag" class="todo-input__select" aria-label="重要程度">
      <option value="high">高优先</option>
      <option value="medium">中优先</option>
      <option value="low">低优先</option>
    </select>
    <button class="todo-input__button" type="submit">添加</button>
  </form>
</template>
