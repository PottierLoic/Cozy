mod task;
mod todo;

use task::Task;
use todo::ToDoList;
use chrono::Local;

fn main() {
    let mut todo = ToDoList::new();
    let deadline = Local::now() + chrono::Duration::days(5);

    todo.add_task(Task::new(1, "Finish Rust project".to_string(), deadline));
    todo.add_task(Task::new(2, "Write blog post".to_string(), deadline));

    todo.complete_task(1);

    println!("{}", todo);
}