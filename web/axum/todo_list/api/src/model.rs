
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use crate::{Error, Result};

#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String
}

#[derive(Deserialize)]
pub struct TodoCreate {
    pub title: String
}

#[derive(Clone)]
pub struct ModelController {
    todos_list: Arc<Mutex<Vec<Option<Todo>>>>
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            todos_list: Arc::default()
        })
    }
}

impl ModelController {
    pub async fn create_todo(&self, todo_create: TodoCreate) -> Result<Todo> {
        let mut todos = self.todos_list.lock().unwrap();

        let id = todos.len() as u64;

        let todo = Todo {
            id,
            title: todo_create.title
        };

        todos.push(Some(todo.clone()));

        Ok(todo)
    }

    pub async fn get_todos (&self) -> Result<Vec<Todo>> {
        let todos_store = self.todos_list.lock().unwrap();

        let todos = todos_store.iter().filter_map(|t| t.clone()).collect();

        Ok(todos)
    }

    pub async fn delete_todo (&self, id: u64) -> Result<Todo> {
        let mut todos = self.todos_list.lock().unwrap();

        let todo = todos.get_mut(id as usize).and_then(|t| t.take());

        todo.ok_or(Error::Fail)
    }

    // pub async fn edit_todo(&self, todo_create: TodoCreate id: u64) -> Result<Todo> {
    //     let mut todos = self.todos_list.lock().unwrap();

    //     for todo in todos.iter_mut() {
    //         if todo.id == Some(id.clone()) {

    //         }
    //     }
    // }
}