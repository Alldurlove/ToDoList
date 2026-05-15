export type ReminderChannel = "system";
export type DueTag = "today" | "within5Hours" | "threeDays" | "thisWeek" | "thisMonth" | "longTerm";
export type PriorityTag = "low" | "medium" | "high";

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
  completedAt?: string;
  dueTag?: DueTag;
  priorityTag?: PriorityTag;
  reminder?: ReminderConfig;
}

export interface CreateTodoInput {
  title: string;
  dueTag?: DueTag;
  priorityTag?: PriorityTag;
}
