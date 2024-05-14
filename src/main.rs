mod task;
use task::Task;

fn main() {
  let mut task = Task::new(1, String::from("Buy milk"), "2024-05-12".parse().unwrap());
  task.complete();
  println!("{}", task);
}
