use std::env;
use std::io::{ self, Write };
use std::fs::{ self, OpenOptions, read_to_string };
use dirs::home_dir;
use owo_colors::OwoColorize;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "add" if args.len() > 2 => add_task(args[2..].join(" ")),
        "done" if args.len() > 2 => {
            if let Ok(index) = args[2].parse::<usize>() {
                finish_task(index);
            } else {
                println!("Invalid task index: {}", args[2]);
            }
        },
        "list" => list_task(),
        "help" => eprint!("Usage: 'todo add <task>' or 'todo list' to show list"),
        "test" => println!("{}", "banana".strikethrough()),
        _ => list_task(),
    }
}

pub fn add_task(task: String) {
    let dir_path = home_dir().unwrap().join(".todo/");

    //create directory if it doesn't exist!
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)
            .expect("Failed to create directory");
    }

    let path = &dir_path.join("todos.txt");
    let msg: &str = "Failed to open file";

    let line = format!("* {}\n", task);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .expect(msg);

    file.write_all(line.as_bytes())
        .expect(msg);
}

pub fn finish_task(index: usize) {
    let dir_path = home_dir().unwrap().join(".todo/");
    let path = &dir_path.join("todos.txt");

    let content = read_to_string(&path)
        .expect("Failed to read file");

    let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();

    if index < lines.len() {
        let task = &mut lines[index - 1];

        if task.starts_with("*") {
            task.replace_range(0..1, "-");
        } else if task.starts_with("-"){
            task.replace_range(0..1, "*");
        }        
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) 
        .open(path)
        .expect("Failed to open file");

    for line in lines {
        writeln!(file, "{}", line)
            .expect("Failed to update task");
    }

    file.flush()
        .expect("flush err");

    list_task();
}

pub fn list_task() {
    let dir_path = home_dir().unwrap().join(".todo/");
    let path = &dir_path.join("todos.txt");

    let content = read_to_string(&path).unwrap_or_else(|_| String::new());

    if !content.is_empty() {
        for (i, line) in content.lines().enumerate() {
            let (status, task) = line.split_at(2);

            if status == "* " {
                println!("{} {}", i + 1, task);
            } else {
                println!("{} {}", i + 1, task.strikethrough());
            }
        }
    } else {
        println!("No tasks to display");
    }
}