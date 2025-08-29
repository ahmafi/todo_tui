use crate::todo::todo::*;

pub struct TodoList {
    title: String,
    todos: Vec<Todo>,
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

    fn set_status(&mut self, id: u32, status: TodoStatus) {}

    fn show(&self) {
        println!("*** {} ***", self.title);
        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}. {}", i + 1, todo.title)
        }
        println!()
    }
}
