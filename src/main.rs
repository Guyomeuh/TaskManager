use std::io;

struct Task {
    name: String,
    done: bool,
}

fn show_tasks(tasks: &[Task]) {
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.done == true { "[x]" } else { "[ ]" };
        println!("{}. {} {}", i, task.name, status);
    }
}

fn set_done(tasks: &mut [Task], index: usize) {
    match tasks.get_mut(index) {
        Some(task) => task.done = true,
        None => println!("This task does not exist"),
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("1. Add task\n2. See tasks\n3. Mark task as done\n4. Leave");
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
            }
            "2" => show_tasks(&tasks),
            "3" => {
                show_tasks(&tasks);
                println!("What do you want to mark as done ?");
                let mut index = String::new();

                io::stdin()
                    .read_line(&mut index)
                    .expect("Error when reading the task");

                match index.trim().parse::<usize>() {
                    Ok(i) => set_done(&mut tasks, i),
                    Err(_) => println!("You're not giving a number !"),
                }
            }
            "4" => break,
            _ => println!("Invalide choice"),
        }
    }
}
