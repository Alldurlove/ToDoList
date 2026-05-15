<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import TodoInput from "./components/TodoInput.vue";
import TodoList from "./components/TodoList.vue";
import { useTodoStore } from "./stores/todoStore";

const store = useTodoStore();
const pendingItems = computed(() => store.pendingItems.value);
const completedItems = computed(() => store.completedItems.value);
const selectedView = ref<"list" | "calendar">("list");
const selectedDate = ref(new Date().toISOString().slice(0, 10));

function dayKey(value: string) {
  const date = new Date(value);
  const year = date.getFullYear();
  const month = `${date.getMonth() + 1}`.padStart(2, "0");
  const day = `${date.getDate()}`.padStart(2, "0");
  return `${year}-${month}-${day}`;
}

function taskCalendarDate(item: (typeof store.items.value)[number]) {
  if (item.completed && item.completedAt) {
    return dayKey(item.completedAt);
  }
  if (item.dueAt) {
    return dayKey(item.dueAt);
  }
  return dayKey(item.createdAt);
}

const calendarItems = computed(() =>
  store.items.value.filter((item) => taskCalendarDate(item) === selectedDate.value)
);

const systemDateLabel = computed(() =>
  new Date(store.systemTime.value).toLocaleString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit"
  })
);

let timer: number | null = null;

onMounted(() => {
  void store.refresh();
  timer = window.setInterval(() => {
    void store.syncSystemTime();
  }, 60_000);
});

onUnmounted(() => {
  if (timer !== null) {
    window.clearInterval(timer);
  }
});
</script>

<template>
  <main class="app">
    <section class="panel">
      <header class="panel__header">
        <h1 class="panel__title">ToDoList</h1>
        <p class="panel__subtitle">轻量、简约、专注完成</p>
        <p class="panel__time">系统时间：{{ systemDateLabel }}</p>
      </header>

      <div class="view-switcher">
        <button
          class="view-switcher__button"
          :class="{ 'view-switcher__button--active': selectedView === 'list' }"
          type="button"
          @click="selectedView = 'list'"
        >
          任务列表
        </button>
        <button
          class="view-switcher__button"
          :class="{ 'view-switcher__button--active': selectedView === 'calendar' }"
          type="button"
          @click="selectedView = 'calendar'"
        >
          日历查看
        </button>
      </div>

      <p v-if="store.error" class="status status--error">{{ store.error }}</p>
      <p v-else-if="store.loading" class="status">加载中...</p>

      <template v-if="selectedView === 'list'">
        <TodoInput @submit="store.addTodo" />

        <section class="todo-group">
          <h2 class="todo-group__title">待完成</h2>
          <TodoList :items="pendingItems" empty-text="太棒了，当前没有待办" @toggle="store.toggleTodo" @remove="store.removeTodo" />
        </section>

        <section class="todo-group">
          <h2 class="todo-group__title">今日已完成</h2>
          <TodoList :items="completedItems" empty-text="今天还没有完成的任务" @toggle="store.toggleTodo" @remove="store.removeTodo" />
        </section>
      </template>

      <template v-else>
        <section class="calendar-panel">
          <label class="calendar-panel__label" for="calendar-date">选择日期</label>
          <input id="calendar-date" v-model="selectedDate" class="calendar-panel__picker" type="date" />
          <TodoList
            :items="calendarItems"
            empty-text="该日期没有任务记录"
            @toggle="store.toggleTodo"
            @remove="store.removeTodo"
          />
        </section>
      </template>
    </section>
  </main>
</template>
