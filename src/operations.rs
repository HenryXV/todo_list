use console::style;
use std::str::FromStr;

#[derive(Debug)]
pub enum Operation {
    Add,
    List,
    Done,
    Remove,
    Reset,
    Sort,
    Exit,
    Help,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Operation::Add),
            "list" => Ok(Operation::List),
            "done" => Ok(Operation::Done),
            "rm" => Ok(Operation::Remove),
            "reset" => Ok(Operation::Reset),
            "sort" => Ok(Operation::Sort),
            "exit" | "quit" => Ok(Operation::Exit),
            "help" | "--help" | "-h" => Ok(Operation::Help),
            _ => Err(()),
        }
    }
}

pub struct Todo {
    task: String,
    done: bool,
}

pub fn process_operation(todo_l: &mut Vec<Todo>, op: Operation, args: &[&str]) {
    match op {
        Operation::Add => {
            if args.len() > 1 {
                let task = args[1..].join(" ");
                todo_l.push(Todo { task, done: false });
            }
        }
        Operation::List => {
            for todo in todo_l {
                let mut task_style = style(todo.task.as_str());

                if todo.done {
                    task_style = task_style.strikethrough();
                }

                println!("{}", task_style);
            }
        }
        Operation::Done => {
            for index in args[1..].iter() {
                let index: Result<usize, _> = index.parse();

                match index {
                    Ok(n) => {
                        let task = todo_l.get_mut(n);
                        if let Some(t) = task {
                            t.done = true;
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
        }
        Operation::Remove => {
            for index in args[1..].iter() {
                let index: Result<usize, _> = index.parse();

                match index {
                    Ok(n) => {
                        if n < todo_l.len() {
                            todo_l.remove(n);
                        }
                    }
                    Err(e) => eprintln!("{e}"),
                }
            }
        }
        Operation::Reset => todo_l.clear(),
        Operation::Sort => todo_l.sort_by(|a, b| a.done.cmp(&b.done)),
        Operation::Exit => (),
        Operation::Help => println!("{}", HELP_MESSAGE),
    }
}

pub fn populate_todo() -> Vec<Todo> {
    let todo1 = Todo {
        task: String::from("first task"),
        done: false,
    };
    let todo2 = Todo {
        task: String::from("second task"),
        done: true,
    };
    let todo3 = Todo {
        task: String::from("third task"),
        done: false,
    };
    let todo4 = Todo {
        task: String::from("fourth task"),
        done: true,
    };

    vec![todo1, todo2, todo3, todo4]
}

const HELP_MESSAGE: &str = "Usage: [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust (credits to https://github.com/sioodmy/todo)
Example: list
Available commands:
    - add [TASK]
        adds new task
        Example: add buy carrots
    - list
        lists all tasks
        Example: list
    - done [INDEX]
        marks task as done
        Example: done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: rm 4
    - reset
        deletes all tasks
    - sort
        sorts completed and uncompleted tasks
        Example: sort
";
