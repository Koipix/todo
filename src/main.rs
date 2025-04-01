use std::env;
use std::io::Write;
use std::fs::{ self, OpenOptions, read_to_string };
use dirs::home_dir;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "add" if args.len() > 2 => add_task(args[2..].join(" ")),
        _ => eprint!("Usage: todo add <task> or 'todo' to show list"),
    }
}

fn add_task(task: String) {
    let dir_path = home_dir().unwrap().join(".todo/");

    //create directory if it doesn't exist!
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)
            .expect("Failed to create directory");
    }

    let path = &dir_path.join("todos.txt");
    let content = read_to_string(&path).unwrap_or_else(|_| String::new());
    let task_count = content.lines().count() + 1;
    let msg: &str = "Failed to open file";

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)
        .expect(msg);

    writeln!(file, "{} {}", task_count, task)
        .expect(msg);
}