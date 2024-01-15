use rusqlite::Connection;

use crate::args::ListCommand;

pub const DATABASE_PATH: &str = "./src/db/database.db3";
#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub status: u8,
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
    // current_todos: Vec<TodoItem>,
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

    pub fn get_todo(&self, id: u32) -> Option<TodoItem> {
        let mut stmt = self
            .db
            .prepare("SELECT * FROM todos WHERE id=:id;")
            .expect("Something went wrong fetching the todo by id.");
        let todo_res = stmt
            .query_map(&[(":id", &id.to_string())], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    status: row.get(2)?,
                })
            })
            .expect("Todo by id does not exist.");
        for todo in todo_res {
            let found_todo = todo.unwrap();
            return Some(found_todo);
        }
        None
    }

    pub fn mark_todo(&self, todo: TodoItem) -> Result<TodoItem, String> {
        let status = if todo.status == 1 { 0 } else { 1 };
        let mut stmt = self
            .db
            .prepare("UPDATE todos SET status = ?1 WHERE id = ?2")
            .expect("Something went wrong while preparing updating the todo.");
        let res = stmt.execute([status, todo.id]);
        if res.is_ok() {
            return Ok(todo);
        }

        Err("Something went wrong".to_owned())
    }

    pub fn print_todos(&self) {
        let todo_list = self.list(None);
        if !todo_list.is_empty() {
            for t in todo_list {
                t.print_todo();
            }
        } else {
            println!("No todos");
        }
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
            // current_todos: Vec::new(),
        }
    }
}

pub fn connect() -> Database {
    let conn: Database = Database::new(Connection::open(DATABASE_PATH).unwrap());
    return conn;
}
