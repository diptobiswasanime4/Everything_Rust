use tokio::net::TcpListener;
use axum::{
    routing::{get, get_service, post, delete, put, patch},
    Json,
    Router,
    middleware,
    response::{Response},
    response::{Html, IntoResponse},
    extract::{Query, Path, State}
};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies, CookieManagerLayer};
use tower_http::services::ServeDir;

mod model;
mod error;
use crate::model::{ModelController, Todo, TodoCreate};
pub use self::error::{Error, Result};

#[derive(Debug, Deserialize)]
struct TodoPayload {
    id: u64,
    name: String,
    completed: bool
}

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_api = routes(mc.clone());

    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/json", get(json_handler))
        .route("/todos", post(todo_handler))
        .nest("/api", routes_api);

        let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
        println!("App running on {:?}", listener.local_addr());
        axum::serve(listener, app).await.unwrap();

        Ok(())
}

async fn hello_handler () -> &'static str {
    "Hello"
}

async fn json_handler () -> Json<Value> {
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    body
}

async fn todo_handler(payload: Json<TodoPayload>) -> Json<Value> {
    let body = Json(json!({
        "todo": {
            "id": payload.id,
            "name": payload.name,
            "completed": payload.completed
        }
    }));
    body
}

async fn create_todo (
    State(mc): State<ModelController>,
    Json(todo_create): Json<TodoCreate>
) -> Result<Json<Todo>> {
    println!("create todo");

    let todo = mc.create_todo(todo_create).await?;

    Ok(Json(todo))
}

async fn todos_list (
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Todo>>> {
    println!("todo_list");

    let todos =  mc.get_todos().await?;

    Ok(Json(todos))
}

async fn delete_todo(
    State(mc): State<ModelController>,
    Path(id): Path<u64>
 ) -> Result<Json<Todo>> {
    println!("delete_todo");

    let todo = mc.delete_todo(id as u64).await?;

    Ok(Json(todo))
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/todos", post(create_todo).get(todos_list))
        .route("/todos/:id", delete(delete_todo))
        .with_state(mc)
}