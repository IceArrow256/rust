use chrono::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
#[derive(Serialize, Deserialize)]
struct Task {
    options: Vec<String>,
    current: usize,
    last_shift: i64,
}

fn get_task_file_path() -> Option<PathBuf> {
    if let Option::Some(proj_dirs) = ProjectDirs::from("com", "IceArrow256", "everyday-task") {
        let data_dir = proj_dirs.data_dir();
        Some(data_dir.join("task.json"))
    } else {
        None
    }
}

fn load_task() -> Option<Task> {
    if let Option::Some(tasks_file_path) = get_task_file_path() {
        if tasks_file_path.exists() {
            let mut data = String::new();
            let mut file = File::open(tasks_file_path).unwrap();
            file.read_to_string(&mut data)
                .expect("Unable to read string");
            serde_json::from_str(&data).ok().unwrap()
        } else {
            Some(Task {
                options: vec!["example".to_string()],
                current: 0,
                last_shift: 0,
            })
        }
    } else {
        None
    }
}

fn save_task(task: Task) {
    if let Option::Some(tasks_file_path) = get_task_file_path() {
        std::fs::create_dir_all(tasks_file_path.parent().unwrap()).ok();
        std::fs::write(tasks_file_path, serde_json::to_string(&task).ok().unwrap()).ok();
    }
}

fn main() {
    let now: DateTime<Local> = Local::now();
    let mut stdout_color = StandardStream::stdout(ColorChoice::Always);
    if let Option::Some(mut task) = load_task() {
        println!("Your current task:");
        if !task.options.is_empty() && task.current < task.options.len() {
            let last_shift: DateTime<Local> = Local.timestamp(task.last_shift, 0);
            if now.date() != last_shift.date() {
                task.current = (task.current + 1) % task.options.len();
            }
            stdout_color
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .ok();
            println!("{}", task.options[task.current],);
            stdout_color
                .set_color(ColorSpec::new().set_reset(true))
                .ok();
        }
        println!("\nYour other tasks: ");
        if task.current + 1 < task.options.len() {
            for i in task.current + 1..task.options.len() {
                println!("{}", task.options[i]);
            }
        }
        if task.current > 0 {
            for i in 0..task.current {
                println!("{}", task.options[i]);
            }
        }
        task.last_shift = now.timestamp();

        save_task(task);
    }
}
