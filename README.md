# Todo CLI

A simple command-line todo application written in Rust.

## Installation

After forking and cloning the repository run these commands.

```bash
cargo install --path .             # Make sure you have rust installed on your machine
```

## Usage

```bash
todocli <command> [options]
```
## Options
* -h, --help: Print help
* -V, --version: Print version


## Lists all todos
```bash
todocli list [options]
```
### Options
* -l, --limit <LIMIT>: Specify the maximum number of todos to show.
* -c, --completed: Show only completed todos.
* -h, --help: Print help


## Add a New Todo Item
```bash
todocli add  [options]
```
### Options
* -t, --title <TITLE>: The title of the todo.
* -c, --completed: Mark the todo as completed upon creation.
* -h, --help: Print help


## Mark/Unmark Todo
```bash
todocli mark  [options]
```
### Options
* -i, --id <ID>: The ID of the todo to mark.
* -h, --help: Print help
