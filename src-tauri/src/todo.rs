use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub label: String,
    pub done: bool,
    pub is_delete: bool,
}

pub struct TodoApp {
    pub conn: Connection,
}

impl TodoApp {
    pub fn new() -> Result<TodoApp> {
        let db_path = "../db.sqlite";
        let conn = Connection::open(db_path)?;
        Ok(TodoApp { conn })
    }

    pub fn get_todo(&self, id: String) -> Result<Todo> {
        let mut stmt = self.conn.prepare("SELECT * FROM todo WHERE id = ?")?;
        let mut rows = stmt.query_map(&[&id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                label: row.get(1)?,
                done: row.get(2)?,
                is_delete: row.get(3)?,
            })
        })?;
        let todo = rows.next().unwrap()?;
        Ok(todo)
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>> {
        let mut stmt = self.conn.prepare("SELECT * FROM Todo").unwrap();
        let todos_iter = stmt.query_map([], |row| {
            let done = row.get::<usize, i32>(2).unwrap() == 1;
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
        match self
            .conn
            .execute("INSERT INTO Todo (id, label) VALUES (?, ?)", [id, label])
        {
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

    pub fn update_todo(&self, todo: Todo) -> bool {
        println!("{:?}", todo);
        let Todo {
            label,
            done,
            is_delete,
            id,
        } = todo;
        let done = if done == true { 1 } else { 0 };
        let is_delete = if is_delete == true { 1 } else { 0 };
        match self.conn.execute(
            "UPDATE Todo
        SET label = ?1, done = ?2, isDelete = ?3 WHERE id = ?4",
            [label, done.to_string(), is_delete.to_string(), id],
        ) {
            Ok(update) => {
                println!("{}", update);
                true
            }
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    }
}
