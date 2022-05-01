#![cfg_attr(
    all(not(debug_assertions), target_os = "w&mut &mut indows"),
    windows_subsystem = "windows"
)]
use rusqlite::{Connection, Result};
use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    id: String,
    label: String,
    done: bool,
    is_delete: bool,
}

pub struct TodoApp {
    conn: Connection,
}

impl TodoApp {
    pub fn new() -> Result<TodoApp> {
        let db_path = "./db.sqlite";
        let conn = Connection::open(db_path)?;
        Ok(TodoApp { conn })
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Todo").unwrap();
        let todos_iter = stmt.query_map([], |row| {
            let done = row.get::<usize, i32>(3).unwrap() == 1;
            let is_delete = row.get::<usize, i32>(3).unwrap() == 1;

            Ok(Todo {
                id: row.get(0)?,
                label: row.get(1)?,
                done,
                is_delete,
            })
        })?;
        let mut todos: Vec<Todo> = Vec::new();

        for todo in todos_iter {
            todos.push(todo?);
        }

        Ok(todos)
    }
    pub fn new_todo(&self, todo: Todo) {
        //TODO
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_todos() -> Vec<Todo> {
    let todo = TodoApp::new().unwrap();
    todo.get_todos().unwrap()
}
