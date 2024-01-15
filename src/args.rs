use clap::{Args, Parser, Subcommand};

use crate::db::Database;

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
                let todo_list = db.list(Some(&cmd));
                if !todo_list.is_empty() {
                    for todo in todo_list {
                        todo.print_todo();
                    }
                } else {
                    println!("No todos");
                }
            }
            CommandType::Add(cmd) => {
                let db = db.insert_todo(&cmd.title, cmd.completed);
                db.print_todos();
            }
            // TODO: mark todo checked or uncheck
            CommandType::Mark(cmd) => {
                match db.get_todo(cmd.id) {
                    Some(todo) => match db.mark_todo(todo) {
                        Ok(td) => {
                            println!("Todo {} updated sucessfully!", td.id);
                            println!("----------------------------------------");
                            db.print_todos();
                        }
                        Err(err) => {
                            panic!("{}", err);
                        }
                    },
                    None => panic!("Todo by id does not exist."),
                };
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// List all todos with an option (todocli list --limit 5)
    List(ListCommand),

    /// Add a todo item use -t or --title flag: example (todocli add --title "New todo"), to automatically checked the added todo just add -c or --completed flag
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
