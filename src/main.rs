use std::str::FromStr;

use console::style;
use console::Term;

use crate::operations::{populate_todo, Todo};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use operations::Operation;

mod operations;

fn main() {
    let mut todo_list: Vec<Todo> = populate_todo();
    let terminal = Term::stdout();

    terminal.set_title("Todo list");

    let welcome_message = style("Welcome to the todo list app!").cyan();

    terminal
        .write_line(welcome_message.to_string().as_str())
        .expect("Failed to print welcome message.");

    let type_command_message = style("Type in a command").cyan();

    loop {
        let command: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(type_command_message.to_string().as_str())
            .interact_text_on(&terminal)
            .unwrap_or("".to_string());

        let args: Vec<&str> = command.split_whitespace().collect();

        let operation = args.first();
        if let Some(op) = operation {
            let operation_enum = Operation::from_str(op);

            match operation_enum {
                Ok(op) => match op {
                    Operation::Exit => break,
                    _ => operations::process_operation(&mut todo_list, op, &args),
                },
                Err(()) => eprintln!(
                    "{} {}",
                    style(op).underlined().bold().to_string().as_str(),
                    style("command is not valid").red().to_string().as_str()
                ),
            }
        }
    }
}
