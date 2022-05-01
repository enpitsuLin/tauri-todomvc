#![cfg_attr(
    all(not(debug_assertions), target_os = "w&mut &mut indows"),
    windows_subsystem = "windows"
)]
use rusqlite::{Connection, Result};

struct Todo {
    id: String,
    label: String,
    done: bool,
    isDelete: bool,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello_world])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn hello_world() -> bool {
    let conn = connection_database();
    create_table(conn)
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
