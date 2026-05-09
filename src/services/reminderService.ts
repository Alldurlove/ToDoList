import type { Todo } from "../types/todo";

export interface ReminderService {
  schedule(todo: Todo): Promise<void>;
  cancel(todoId: string): Promise<void>;
  resync(todos: Todo[]): Promise<void>;
}

/**
 * Default no-op implementation.
 * Later this can be replaced with OS notification adapters.
 */
export class NoopReminderService implements ReminderService {
  async schedule(_todo: Todo): Promise<void> {}

  async cancel(_todoId: string): Promise<void> {}

  async resync(_todos: Todo[]): Promise<void> {}
}

export const reminderService: ReminderService = new NoopReminderService();
