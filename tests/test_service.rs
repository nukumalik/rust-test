use std::sync::Arc;

use async_trait::async_trait;
use mockall::{mock, predicate::*};
use rust_test::{
    entitty::Todo,
    repository::TodoRepository,
    service::{TodoService, TodoServiceImpl},
};
use tokio::sync::Mutex;

mock! {
    pub TodoRepositoryImpl {}

    #[async_trait]
    impl TodoRepository for TodoRepositoryImpl {
        async fn find_many(&self, limit: u32, offset: u32) -> Vec<Todo>;
    }
}

fn mock_find_many() -> MockTodoRepositoryImpl {
    let mut mock = MockTodoRepositoryImpl::new();
    mock.expect_find_many()
        .with(eq(10), eq(0))
        .returning(|_, _| {
            vec![
                Todo::new(1, "Test 1".to_owned(), false),
                Todo::new(2, "Test 2".to_owned(), false),
                Todo::new(3, "Test 3".to_owned(), false),
                Todo::new(4, "Test 4".to_owned(), false),
                Todo::new(5, "Test 5".to_owned(), false),
            ]
        });
    mock.expect_find_many()
        .with(eq(10), eq(10))
        .returning(|_, _| vec![]);
    mock
}

#[tokio::test]
async fn test_todo_service_get_todo_list() {
    let mock = mock_find_many();
    let todo_service = TodoServiceImpl::new(Arc::new(Mutex::new(mock)));
    let todos = todo_service.get_todo_list(Some(1)).await;
    assert_eq!(todos.len(), 5);

    let todos = todo_service.get_todo_list(Some(2)).await;
    assert_eq!(todos.len(), 0);
}
