use rusqlite::Connection;

use crate::args::ListCommand;

pub const DATABASE_PATH: &str = "./src/db/database.db3";
#[derive(Debug, Clone)]
pub struct TodoItem {
    id: u32,
    title: String,
    status: u8,
}

impl TodoItem {
    pub fn print_todo(&self) {
        let complete = if self.status == 1 { "x" } else { " " };
        println!("#{}[{}] - {}", self.id, complete, self.title);
        println!("----------------------------------------");
    }
}

// struct TodoList {
//     todos: Vec<TodoItem>,
// }

// impl TodoList {
//     fn new(todos: &Vec<TodoItem>) -> Self {
//         Self {
//             todos: todos.to_vec(),
//         }
//     }

//     fn add(mut self, todo: TodoItem) -> Self {
//         self.todos.push(todo);
//         return self;
//     }

//     pub fn list(&self) -> &Vec<TodoItem> {
//         &self.todos
//     }
// }

#[derive(Debug)]
pub struct Database {
    db: Connection,
    current_todos: Vec<TodoItem>,
}

impl Database {
    pub fn list(&self, params: Option<&ListCommand>) -> Vec<TodoItem> {
        let mut sql = String::from("SELECT * FROM todos");
        // let Some(val) = params;
        if params.is_some_and(|p| p.limit.is_some()) {
            let limit_count = params.unwrap().limit.unwrap();
            let limit = format!("LIMIT {}", limit_count);
            sql += &limit;
        }
        let mut stmt = self
            .db
            .prepare(&sql)
            .expect("Something went wrong while getting the todo list");

        let todo_result = stmt
            .query_map([], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    status: row.get(2)?,
                })
            })
            .unwrap();

        let mut todos: Vec<TodoItem> = Vec::new();
        for todo in todo_result {
            todos.push(todo.unwrap());
        }

        return todos;
    }

    pub fn insert_todo(&self, title: &str, is_completed: bool) -> &Self {
        let res = self
            .db
            .execute(
                "INSERT INTO todos (title, status) VALUES (?1, ?2)",
                (title, is_completed),
            )
            .expect("Something went wrong while inserting todo");
        println!("{:?}", res);
        return self;
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
        Self {
            db,
            current_todos: Vec::new(),
        }
    }
}

pub fn connect() -> Database {
    let conn: Database = Database::new(Connection::open(DATABASE_PATH).unwrap());
    return conn;
}
