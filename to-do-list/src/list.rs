use crate::task::Task;
use std::collections::HashMap;

pub struct ToDoList{
    tasks: HashMap<u32, Task>,
}

impl ToDoList {

    pub fn new() -> ToDoList {
        ToDoList {
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task){
        self.tasks.insert(task.id, task);
    }

    pub fn remove_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.remove(&id){
            println!("Removed Task: {}", task);
        } else {
            println!("Task with id {} not found", id);
        }
    }

    pub fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.get_mut(&id){
            task.mark_as_completed();
            println!("Marked Task as Completed: {}", task);
        } else {
            println!("Task with id {} not found", id);
        }
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found!!");
        } else {
            for task in self.tasks.values(){
                println!("{}", task);
            }
        }
    }
}