use std::sync::Arc;

use rust_test::repository::{TodoRepository, TodoRepositoryImpl};
use sqlx::SqlitePool;
use tokio::sync::Mutex;

#[sqlx::test(fixtures("./todo_fixture.sql"))]
async fn test_find_many(db: SqlitePool) -> sqlx::Result<()> {
    let todo_repository = TodoRepositoryImpl::new(Arc::new(Mutex::new(db)));
    let todos = todo_repository.find_many(10, 0).await;
    assert_eq!(todos.len(), 5);

    let todos = todo_repository.find_many(3, 0).await;
    assert_eq!(todos.len(), 3);

    let todos = todo_repository.find_many(3, 3).await;
    assert_eq!(todos.len(), 2);

    let todos = todo_repository.find_many(3, 6).await;
    assert_eq!(todos.len(), 0);

    Ok(())
}
