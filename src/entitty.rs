use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn new(id: i32, title: String, done: bool) -> Self {
        Self { id, title, done }
    }
}
