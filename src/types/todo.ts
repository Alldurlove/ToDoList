export type ReminderChannel = "system";

export interface ReminderConfig {
  enabled: boolean;
  triggerAt?: string;
  channel?: ReminderChannel;
}

export interface Todo {
  id: string;
  title: string;
  completed: boolean;
  createdAt: string;
  updatedAt: string;
  dueAt?: string;
  reminder?: ReminderConfig;
}

export interface CreateTodoInput {
  title: string;
  dueAt?: string;
}
