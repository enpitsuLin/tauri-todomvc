#![cfg_attr(
    all(not(debug_assertions), target_os = "w&mut &mut indows"),
    windows_subsystem = "windows"
)]
mod todo;
use todo::{Todo, TodoApp};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_todos])
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
