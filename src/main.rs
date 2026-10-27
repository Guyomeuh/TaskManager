use std::{fs, io};

const FILE: &str = "tasks.txt";

struct Task {
    name: String,
    done: bool,
}

fn save_tasks(tasks: &[Task]) {
    let mut content = String::new();

    for task in tasks {
        let done = if task.done { 1 } else { 0 };
        content.push_str(&format!("{};{}\n", done, task.name));
    }
    fs::write(FILE, content).expect("Error when saving");
}

fn load_tasks() -> Vec<Task> {
    let content = match fs::read_to_string(FILE) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut tasks = Vec::new();
    for line in content.lines() {
        if let Some((done, name)) = line.split_once(';') {
            tasks.push(Task {
                name: name.to_string(),
                done: done == "1",
            })
        }
    }
    tasks
}

fn delete_all_done(tasks: &mut Vec<Task>) {
    for i in (0..tasks.len()).rev() {
        if tasks[i].done == true {
            tasks.remove(i);
        }
    }
}
fn delete_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        tasks.remove(index);
    } else {
        println!("This task does not exist");
    }
}

fn show_tasks(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done == true { "[✓]" } else { "[ ]" };

        print!("{}: {} {}\n", i + 1, status, task.name);
    }
}

fn set_done(tasks: &mut [Task], index: usize) {
    match tasks.get_mut(index) {
        Some(task) => task.done = true,
        None => println!("This task does not exist"),
    }
}

fn clear_screen() {
    clearscreen::clear().expect("failed to clear screen");
}

fn main() {
    clear_screen();
    let mut tasks = load_tasks();
    loop {
        println!(
            "1. Add task\n2. See tasks\n3. Mark task as done\n4. Delete a task\n5. Delete all completed tasks\n6. Leave\n"
        );
        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Error when reading the number.");

        match choice.trim() {
            "1" => {
                println!("What do you want to add ?");
                let mut new_task = String::new();

                io::stdin()
                    .read_line(&mut new_task)
                    .expect("Error when reading the task");
                let t = Task {
                    name: new_task.trim().to_string(),
                    done: false,
                };
                tasks.push(t);
                save_tasks(&tasks);
                clear_screen();
            }

            "2" => {
                clear_screen();
                show_tasks(&tasks);
                print!("\nPress enter to go to the menu\n");
                let mut buffer = String::new();

                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Error when reading the input");

                clear_screen();
            }

            "3" => {
                show_tasks(&tasks);
                println!("What do you want to mark as done ?");
                let mut index = String::new();

                io::stdin()
                    .read_line(&mut index)
                    .expect("Error when reading the task");

                match index.trim().parse::<usize>() {
                    Ok(i) if i > 0 => {
                        set_done(&mut tasks, i - 1);
                        save_tasks(&tasks);
                        clear_screen();
                    }
                    Ok(_) => println!("This task does not exist"),
                    Err(_) => println!("You're not giving a number !"),
                }
            }

            "4" => {
                show_tasks(&tasks);
                println!("What do you want to delete ?");
                let mut index = String::new();

                io::stdin()
                    .read_line(&mut index)
                    .expect("Error when reading the task");

                match index.trim().parse::<usize>() {
                    Ok(i) if i > 0 => {
                        delete_task(&mut tasks, i - 1);
                        save_tasks(&tasks);
                        clear_screen();
                    }
                    Ok(_) => println!("This task does not exist"),
                    Err(_) => println!("You're not giving a number !"),
                }
            }

            "5" => {
                delete_all_done(&mut tasks);
                save_tasks(&tasks);
                clear_screen();
            }

            "6" => break,

            _ => {
                clear_screen();
                println!("/!\\ Invalide choice /!\\")
            }
        }
    }
}
