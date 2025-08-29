use uuid::Uuid;

#[derive(PartialEq)]
pub enum TodoStatus {
    TODO,
    DOING,
    DONE,
}

pub struct Todo {
    id: Uuid,
    pub title: String,
    pub status: TodoStatus,
}

impl Todo {
    pub fn new(&self, title: &str, status: TodoStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            status: status,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
