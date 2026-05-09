import { invoke } from "@tauri-apps/api/core";
import type { CreateTodoInput, Todo } from "../types/todo";

export interface TodoApi {
  listTodos(): Promise<Todo[]>;
  createTodo(input: CreateTodoInput): Promise<Todo>;
  setCompleted(id: string, completed: boolean): Promise<Todo>;
  deleteTodo(id: string): Promise<void>;
}

class TauriTodoApi implements TodoApi {
  async listTodos(): Promise<Todo[]> {
    return invoke<Todo[]>("list_todos");
  }

  async createTodo(input: CreateTodoInput): Promise<Todo> {
    return invoke<Todo>("create_todo", { input });
  }

  async setCompleted(id: string, completed: boolean): Promise<Todo> {
    return invoke<Todo>("set_completed", { id, completed });
  }

  async deleteTodo(id: string): Promise<void> {
    return invoke<void>("delete_todo", { id });
  }
}

export const todoApi: TodoApi = new TauriTodoApi();
