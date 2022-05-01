#![cfg_attr(
    all(not(debug_assertions), target_os = "w&mut &mut indows"),
    windows_subsystem = "windows"
)]
mod todo;
use todo::{Todo, TodoApp};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_todos, new_todo, toggle_done])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_todos() -> Vec<Todo> {
    let app = TodoApp::new().unwrap();
    let todos = app.get_todos().unwrap();
    app.conn.close();
    todos
}

#[tauri::command]
fn new_todo(todo: Todo) -> bool {
    let app = TodoApp::new().unwrap();
    let result = app.new_todo(todo);
    app.conn.close();
    result
}

#[tauri::command]
fn toggle_done(id: String) -> bool {
    let app = TodoApp::new().unwrap();
    let Todo {
        id,
        label,
        done,
        is_delete,
    } = app.get_todo(id).unwrap();
    app.update_todo(Todo {
        id,
        label,
        done: !done,
        is_delete,
    })
}
