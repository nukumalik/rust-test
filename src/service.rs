use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{entitty::Todo, repository::TodoRepository, utils::get_offset_from_page};

#[async_trait]
pub trait TodoService {
    async fn get_todo_list(&self, page: Option<u32>) -> Vec<Todo>;
}

pub struct TodoServiceImpl {
    todo_repository: Arc<Mutex<dyn TodoRepository + Send + Sync>>,
}

impl TodoServiceImpl {
    pub fn new(todo_repository: Arc<Mutex<dyn TodoRepository + Send + Sync>>) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl TodoService for TodoServiceImpl {
    async fn get_todo_list(&self, page: Option<u32>) -> Vec<Todo> {
        let todo_repository = self.todo_repository.lock().await;
        let todos = todo_repository
            .find_many(10, get_offset_from_page(page))
            .await;
        return todos;
    }
}
