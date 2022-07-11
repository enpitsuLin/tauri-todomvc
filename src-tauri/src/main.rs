#![cfg_attr(
    all(not(debug_assertions), target_os = "w&mut &mut indows"),
    windows_subsystem = "windows"
)]
mod todo;
use std::sync::Mutex;
use todo::{Todo, TodoApp};

struct AppState {
    app: Mutex<TodoApp>,
}

fn main() {
    let app = TodoApp::new().unwrap();
    tauri::Builder::default()
        .manage(AppState {
            app: Mutex::from(app),
        })
        .invoke_handler(tauri::generate_handler![
            get_todos,
            new_todo,
            toggle_done,
            update_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_todos(state: tauri::State<AppState>) -> Vec<Todo> {
    let app = state.app.lock().unwrap();
    let todos = app.get_todos().unwrap();
    todos
}

#[tauri::command]
fn new_todo(state: tauri::State<AppState>, todo: Todo) -> bool {
    let app = state.app.lock().unwrap();
    let result = app.new_todo(todo);
    result
}

#[tauri::command]
fn update_todo(state: tauri::State<AppState>, todo: Todo) -> bool {
    let app = state.app.lock().unwrap();
    let result = app.update_todo(todo);
    result
}

#[tauri::command]
fn toggle_done(state: tauri::State<AppState>, id: String) -> bool {
    let app = state.app.lock().unwrap();
    let Todo {
        id,
        label,
        done,
        is_delete,
    } = app.get_todo(id).unwrap();
    let result = app.update_todo(Todo {
        id,
        label,
        done: !done,
        is_delete,
    });
    result
}
