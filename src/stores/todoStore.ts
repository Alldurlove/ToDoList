import { computed, ref } from "vue";
import { reminderService } from "../services/reminderService";
import { todoApi } from "../services/todoApi";
import type { CreateTodoInput, Todo } from "../types/todo";

const items = ref<Todo[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

export function useTodoStore() {
  const pendingItems = computed(() => items.value.filter((item) => !item.completed));
  const completedItems = computed(() => items.value.filter((item) => item.completed));

  async function refresh() {
    loading.value = true;
    error.value = null;
    try {
      const todos = await todoApi.listTodos();
      items.value = todos;
      await reminderService.resync(todos);
    } catch (err) {
      error.value = err instanceof Error ? err.message : "加载任务失败";
    } finally {
      loading.value = false;
    }
  }

  async function addTodo(input: CreateTodoInput) {
    const normalized = input.title.trim();
    if (!normalized) {
      return;
    }
    error.value = null;
    try {
      const todo = await todoApi.createTodo({
        title: normalized,
        dueTag: input.dueTag,
        priorityTag: input.priorityTag
      });
      items.value.unshift(todo);
      await reminderService.schedule(todo);
    } catch (err) {
      error.value = err instanceof Error ? err.message : "创建任务失败";
    }
  }

  async function toggleTodo(todo: Todo) {
    error.value = null;
    try {
      const updated = await todoApi.setCompleted(todo.id, !todo.completed);
      items.value = items.value.map((item) => (item.id === updated.id ? updated : item));
      if (updated.completed) {
        await reminderService.cancel(updated.id);
      } else {
        await reminderService.schedule(updated);
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : "更新任务失败";
    }
  }

  async function removeTodo(todo: Todo) {
    error.value = null;
    try {
      await todoApi.deleteTodo(todo.id);
      items.value = items.value.filter((item) => item.id !== todo.id);
      await reminderService.cancel(todo.id);
    } catch (err) {
      error.value = err instanceof Error ? err.message : "删除任务失败";
    }
  }

  return {
    items,
    pendingItems,
    completedItems,
    loading,
    error,
    refresh,
    addTodo,
    toggleTodo,
    removeTodo
  };
}
