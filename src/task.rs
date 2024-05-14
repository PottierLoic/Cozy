use chrono::{NaiveDate, Local};
use std::fmt;

#[derive(Debug)]
pub enum TaskStatus {
  Pending,
  Completed,
}

pub struct Task {
  pub id: u32,
  pub description: String,
  pub status: TaskStatus,
  pub deadline: NaiveDate,
}

impl Task {
  pub fn new(id: u32, description: String, deadline: NaiveDate) -> Self {
    Task {
      id,
      description,
      status: TaskStatus::Pending,
      deadline,
    }
  }

  pub fn complete(&mut self) {
    self.status = TaskStatus::Completed;
  }

  pub fn is_overdue(&self) -> bool {
    self.deadline < Local::now().naive_local().into()
  }
}

impl fmt::Display for Task {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Task {}: {} (Status: {:?}, Deadline: {}) ", self.id, self.description, self.status, self.deadline)
  }
}