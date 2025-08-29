use std::fmt::Debug;

use uuid::Uuid;

use crate::todo::todo::*;

pub struct TodoList {
    title: String,
    todos: Vec<Todo>,
}

impl Debug for TodoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TodoList Debug not implemented")
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self {
            title: String::from("Default"),
            todos: Vec::new(),
        }
    }
}

impl TodoList {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.into(),
            todos: Vec::new(),
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn remove(&mut self, id: &Uuid) -> Result<(), ()> {
        if let Some(pos) = self.todos.iter().position(|x| x.id() == id) {
            self.todos.remove(pos);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_by_status(&self, status: TodoStatus) -> Vec<&Todo> {
        self.todos.iter().filter(|t| t.status == status).collect()
    }

    fn set_status(&mut self, id: u32, status: TodoStatus) {}

    fn show(&self) {
        println!("*** {} ***", self.title);
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}. {}", i + 1, todo.title)
        }
        println!()
    }
}
