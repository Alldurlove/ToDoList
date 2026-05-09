<script setup lang="ts">
import { ref } from "vue";

const emit = defineEmits<{
  submit: [title: string];
}>();

const title = ref("");

function onSubmit() {
  const value = title.value.trim();
  if (!value) {
    return;
  }
  emit("submit", value);
  title.value = "";
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
    <button class="todo-input__button" type="submit">添加</button>
  </form>
</template>
