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
    pub conn: Connection,
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

    pub fn new_todo(&self, todo: Todo) -> bool {
        let Todo { id, label, .. } = todo;
        match self.conn.execute(
            "INSERT INTO Todo (id, label, done, is_delete) VALUES (?, ?, ?, ?)",
            [id, label],
        ) {
            Ok(insert) => {
                println!("{}", insert);
                true
            }
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    }

    pub fn update_todo(&self, todo: Todo) {
        //TODO
    }
}
