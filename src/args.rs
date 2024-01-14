use clap::{Args, Parser, Subcommand};

use crate::db::{Database, TodoItem};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub command_type: CommandType,
}

impl TodoArgs {
    pub fn handle(self, db: &Database) {
        match self.command_type {
            CommandType::List(cmd) => {
                let todo_list = db.list(&cmd);
                if !todo_list.is_empty() {
                    for todo in todo_list {
                        todo.print_todo();
                    }
                } else {
                    println!("No todos");
                }
            }
            // TODO: add todo!
            CommandType::Add(cmd) => {
                // cmd.title
            }
            // TODO: mark todo checked or uncheck
            CommandType::Mark(cmd) => {}
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// List all todos with an option (todocli list --limit 5)
    List(ListCommand),

    /// Add a todo item. example (todocli add "New todo")
    Add(AddCommand),

    /// Mark a todo item using the id
    Mark(MarkCommand),
}

#[derive(Debug, Args)]
pub struct ListCommand {
    /// Specify the limit you want to show
    #[arg(short, long)]
    pub limit: Option<u32>,

    /// Show todo that are completed
    #[arg(short, long)]
    pub completed: bool,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The title of the todo (--title "Some todo")
    #[arg(short, long)]
    title: String,

    // You can also mark the added todo marked as completed by adding -c or --c flag
    #[arg(short, long)]
    completed: bool,
}

#[derive(Debug, Args)]
pub struct MarkCommand {
    /// Specify the id of the todo you want to mark or unmark
    #[arg(short, long)]
    id: u32,
}
