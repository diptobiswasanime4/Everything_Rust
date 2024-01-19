#[macro_use]
extern crate rocket;

use rocket::{http::Status, serde::json::Json, State, response::status::Custom};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub name: String,
    pub completed: Option<bool>
}

#[derive(Serialize, Debug)]
pub struct TodoListResp {
    pub status: String,
    pub res: usize,
    pub todos: Vec<Todo>
}

#[derive(Serialize, Debug)]
pub struct SingleTodoResp {
    pub status: String,
    pub data: TodoData
}

#[derive(Serialize, Debug)]
pub struct TodoData {
    pub todo: Todo
}

pub struct AppState {
    pub todo_list: Arc<Mutex<Vec<Todo>>>
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            todo_list: Arc::new(Mutex::new(Vec::new()))
        }
    }
}


#[get("/health_checker")]
fn health_checker() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "Todo List in Rust";

    let resp = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string()
    };
    Ok(Json(resp))
}

#[get("/todos")]
fn get_todos (data: &State<AppState>) -> Result<Json<TodoListResp>, Status> {
    let vec = data.todo_list.lock().unwrap();

    let todos: Vec<Todo> = vec.clone().into_iter().collect();

    let resp = TodoListResp {
        status: "success".to_string(),
        res: todos.len(),
        todos
    };

    Ok(Json(resp))
}

// #[post("/todos", data = "<body>")]
// fn create_todo (mut body: Json<Todo>, data: &State<AppState>) -> Result<Json<SingleTodoResp>, Custom<Json<GenericResponse>>> {
//     let mut vec = data.todo_list.lock().unwrap();

//     for todo in vec.iter() {
//         if todo.name == body.name {
//             let err_resp = GenericResponse {
//                 status: "failure".to_string(),
//                 message: format!("Todo with name: '{}' already exists", todo.name),
//             };
//             return Err(Custom::Conflict, Json(err_resp));
//         }
//     }

//     let todo_id = "5";

//     body.id = Some(todo_id.to_string());
//     body.completed = Some(false);

//     let todo = body.to_owned();

//     vec.push(body.into_inner());

//     let resp = TodoListResp {
//         status: "success".to_string(),
//         data: TodoData {
//             todo: todo.into_inner(),
//         }
//     };

//     Ok(Json(resp))
// }

#[post("/todos", data = "<body>")]
fn create_todo_handler(
    mut body: Json<Todo>,
    data: &State<AppState>,
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {
    let mut vec = data.todo_db.lock().unwrap();

    for todo in vec.iter() {
        if todo.title == body.title {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with title: '{}' already exists", todo.title),
            };
            return Err(Custom(Status::Conflict, Json(error_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body.into_inner());

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData {
            todo: todo.into_inner(),
        },
    };

    Ok(Json(json_response))
}

#[launch]
fn rocket() -> _ {
    let app_state = AppState::init();

    rocket::build()
    .manage(app_state)
    .mount("/api", routes![health_checker, get_todos, create_todo_handler])
}

