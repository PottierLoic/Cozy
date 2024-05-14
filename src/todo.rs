use crate::task::{Task, TaskStatus};
use std::{vec::Vec, fmt};

pub struct ToDoList {
  tasks: Vec<Task>,
}

impl ToDoList {
  pub fn new() -> Self {
    ToDoList {
      tasks: Vec::new(),
    }
  }

  pub fn add_task(&mut self, task: Task) {
    self.tasks.push(task);
  }

  pub fn remove_task(&mut self, task_id: u32) {
    self.tasks.retain(|task| task.id != task_id);
  }

  pub fn complete_task(&mut self, task_id: u32) {
    if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
      task.complete();
    }
  }

  pub fn get_task(&self, task_id: u32) -> Option<&Task> {
    self.tasks.iter().find(|&task| task.id == task_id)
  }

  pub fn list_tasks(&self) -> &Vec<Task> {
    &self.tasks
  }

  pub fn list_overdue_tasks(&self) -> Vec<&Task> {
    self.tasks.iter().filter(|&task| task.is_overdue()).collect()
  }
}

impl fmt::Display for ToDoList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let tasks_str = self.tasks.iter()
      .map(|task| format!("{}", task))
      .collect::<Vec<_>>()
      .join("\n");
    if tasks_str.is_empty() {
      write!(f, "The to-do list is currently empty.")
    } else {
      write!(f, "To-Do List:\n{}", tasks_str)
    }
  }
}