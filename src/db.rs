use rusqlite::Connection;

use crate::args::ListCommand;

#[derive(Clone)]
pub struct TodoItem {
    title: String,
    completed: u8,
}

impl TodoItem {
    pub fn print_todo(&self) {
        let complete = if self.completed == 1 { "x" } else { " " };
        println!("* [{}] Title: {}", complete, self.title);
        println!("----------------------------------------");
    }
}

pub const DATABASE_PATH: &str = "./src/db/database.db3";

#[derive(Debug)]
pub struct Database {
    db: Connection,
}

impl Database {
    pub fn list(&self, params: &ListCommand) -> Vec<TodoItem> {
        let mut sql = String::from("SELECT * FROM todos");
        if params.limit.is_some() {
            let limit_count = params.limit.unwrap();
            let limit = format!("LIMIT {}", limit_count);
            sql += &limit;
        }
        let mut stmt = self
            .db
            .prepare(&sql)
            .expect("Something went wrong while getting the todo list");
        let mut todos: Vec<TodoItem> = Vec::new();
        let _ = stmt.query_map([], |row| {
            let todo = TodoItem {
                title: row.get(0)?,
                completed: row.get(1)?,
            };
            todos.push(todo.to_owned());
            Ok(todo)
        });
        return todos;
    }

    fn new(db: Connection) -> Self {
        db.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                status BOOLEAN NOT NULL DEFAULT 0
            )",
            [],
        )
        .unwrap();
        Self { db }
    }
}

pub fn connect() -> Database {
    // let conn = Connection::open_in_memory();
    let conn: Database = Database::new(Connection::open(DATABASE_PATH).unwrap());

    return conn;
}
