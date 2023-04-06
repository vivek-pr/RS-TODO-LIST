use std::fs::File;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    complete: bool,
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    tasks.push(Task {
        description: description,
        complete: false,
    });
}

fn complete_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), String> {
    if index >= tasks.len() {
        return Err(String::from("Invalid task index"));
    }

    tasks[index].complete = true;
    Ok(())
}

fn view_tasks(tasks: &Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.complete { "✓" } else { " " };
        println!("{} [{}] {}", i + 1, status, task.description);
    }
}

fn remove_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), String> {
    if index >= tasks.len() {
        return Err(String::from("Invalid task index"));
    }

    tasks.remove(index);
    Ok(())
}

fn read_tasks_from_file() -> Result<Vec<Task>, String> {
    let file = match File::open("tasks.json") {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };

    match serde_json::from_reader(file) {
        Ok(tasks) => Ok(tasks),
        Err(_) => Err(String::from("Failed to read tasks from file")),
    }
}

fn write_tasks_to_file(tasks: &Vec<Task>) -> Result<(), String> {
    let file = match File::create("tasks.json") {
        Ok(file) => file,
        Err(_) => return Err(String::from("Failed to create tasks file")),
    };

    match serde_json::to_writer_pretty(file, tasks) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to write tasks to file")),
    }
}


fn main() {
    let mut tasks = read_tasks_from_file().unwrap();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts[0];
        let args = parts.get(1).unwrap_or(&"");

        match command {
            "add" => {
                add_task(&mut tasks, String::from(*args));
                write_tasks_to_file(&tasks).unwrap();
            }
            "complete" => {
                let index = args.parse::<usize>().unwrap() - 1;
                complete_task(&mut tasks, index).unwrap();
                write_tasks_to_file(&tasks).unwrap();
            }
            "view" => {
                view_tasks(&tasks);
            }
            "remove" => {
                let index = args.parse::<usize>().unwrap() - 1;
                remove_task(&mut tasks, index).unwrap();
                write_tasks_to_file(&tasks).unwrap();
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
