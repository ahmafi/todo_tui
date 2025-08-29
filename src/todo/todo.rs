use uuid::Uuid;

pub enum TodoStatus {
    TODO,
    DOING,
    DONE,
}

pub struct Todo {
    id: Uuid,
    title: String,
    status: TodoStatus,
}

impl Todo {
    pub fn new(&self, title: &str, status: TodoStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            status: status,
        }
    }
}
