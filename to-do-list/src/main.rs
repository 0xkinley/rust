mod task;
mod list;

use list::ToDoList;
use task::Task;

fn main(){

    let mut todo_list = ToDoList::new();

    todo_list.add_task(Task::new(1, "Go to Gym".to_string()));
    todo_list.add_task(Task::new(2, "Practise Rust".to_string()));
    todo_list.add_task(Task::new(3, "Buy Groceries".to_string()));

    println!("Initial To-Do List:");
    todo_list.list_tasks();

    todo_list.complete_task(2);

    println!("\nUpdated To-Do List:");
    todo_list.list_tasks();

    todo_list.remove_task(1);

    println!("\nFinal To-Do List:");
    todo_list.list_tasks();
}