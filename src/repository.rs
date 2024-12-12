use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{query_as, Sqlite, SqlitePool};
use tokio::sync::Mutex;

use crate::entitty::Todo;

#[async_trait]
pub trait TodoRepository {
    async fn find_many(&self, limit: u32, offset: u32) -> Vec<Todo>;
}

pub struct TodoRepositoryImpl {
    db: Arc<Mutex<SqlitePool>>,
}

impl TodoRepositoryImpl {
    pub fn new(db: Arc<Mutex<SqlitePool>>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_many(&self, limit: u32, offset: u32) -> Vec<Todo> {
        let db = self.db.lock().await.clone();
        let sql = "SELECT * FROM todos LIMIT $1 OFFSET $2";
        let result = query_as::<Sqlite, Todo>(sql)
            .bind(limit)
            .bind(offset)
            .fetch_all(&db)
            .await;

        match result {
            Ok(todos) => todos,
            Err(err) => {
                println!("Error TodoRepository::find_many {:?}", err);
                vec![]
            }
        }
    }
}
