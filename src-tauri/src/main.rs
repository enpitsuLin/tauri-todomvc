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
    isDelete: bool,
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
            Ok(Todo {
                id: row.get(0)?,
                label: row.get(1)?,
                done: row.get(2)?,
                isDelete: row.get(3)?,
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
        .invoke_handler(tauri::generate_handler![hello_world, get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello_world() -> bool {
    let conn = connection_database();
    create_table(conn)
}

#[tauri::command]
fn get_todos() -> Vec<Todo> {
    return vec![
        Todo {
            id: "63A53A04-C1BA-4009-6B46-065914C47FC6".to_string(),
            label: "todo1".to_string(),
            done: false,
            isDelete: false,
        },
        Todo {
            id: "866D7AAA-6435-CEDC-5E59-AB3E989E1018".to_string(),
            label: "todo2".to_string(),
            done: false,
            isDelete: false,
        },
    ];
}

fn create_table(conn: Connection) -> bool {
    match conn.execute(
        "CREATE TABLE Todo(
            id          VARCHAR(64)        PRIMARY KEY     UNIQUE,
            label       TEXT        NOT NULL,
            done        NUMERIC     DEFAULT 0,
            isDelete    NUMERIC     DEFAULT 0
        )",
        [],
    ) {
        Ok(_) => {
            println!("Todo Table created");
            true
        }
        Err(err) => {
            if err.to_string() == "table Todo already exists" {
                return true;
            };
            println!("created failed: {}", err);
            false
        }
    }
}

fn insert_todo(conn: Connection, todo: Todo) {
    match conn.execute(
        "INSERT INTO Todo (id, label) VALUES (?1, ?2)",
        [&todo.id, &todo.label],
    ) {
        Ok(insert) => println!("Item inserted {}", insert),
        Err(err) => println!("insert failed: {}", err),
    }
}

fn connection_database() -> Connection {
    let db_path = "./db.sqlite";
    let conn = Connection::open(&db_path);
    let conn = match conn {
        Ok(r) => r,
        Err(error) => {
            panic!("Problem connect the file: {:?}", error)
        }
    };
    conn
}
