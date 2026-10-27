use std::io;

fn show_tasks(tasks: &[String]) {
    for task in tasks.iter() {
        println!("{}", task);
    }
}

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    loop {
        println!("1. Add task\n2. See tasks\n3. Leave");
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
                tasks.push(new_task);
            }
            "2" => show_tasks(&tasks),
            "3" => break,
            _ => println!("Invalide choice"),
        }
    }
}
