use std::{
    env::temp_dir,
    fs::{read_to_string, File, OpenOptions},
    io::Write,
    process::exit,
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "taskman", version = "0.1.0", about = "Simple terminal-based task manager")]
enum Command {
    #[structopt(name = "add", about = "Add a new task")]
    Add {
        #[structopt(help = "The task to add")]
        task: Vec<String>,
    },
    #[structopt(name = "view", about = "View all tasks")]
    View,
    #[structopt(name = "remove", about = "Remove a task")]
    Remove {
        #[structopt(help = "The task to remove")]
        id: Option<i32>,
        #[structopt(long = "all", help = "Remove all tasks")]
        all: bool,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "taskman", version = "0.1.0", about = "Simple terminal-based task manager")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(long, help = "Prints version information")]
    version: bool,
}

struct Task {
    id: i32,
    content: String,
}

fn main() {
    let opt = Opt::from_args();

    if opt.version {
        println!("taskman version 1.0");
        return;
    }

    match opt.command {
        Command::Add { task } => {
            if task.is_empty() {
                println!("No task provided!");
                exit(1);
            }

            let task_content = task.join(" ");

            let mut tasks: Vec<Task> = Vec::new();
            let mut taskfile_path = temp_dir();
            taskfile_path.push("Taskfile");

            if !taskfile_path.exists() {
                match File::create(&taskfile_path) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("ERROR[3]: {error}");
                        exit(1);
                    }
                }
            }

            match read_to_string(&taskfile_path) {
                Ok(taskfile) => {
                    for line in taskfile.lines() {
                        if let Some((id_str, content_str)) = line.split_once(". ") {
                            match id_str.parse::<i32>() {
                                Ok(id) => {
                                    let content = content_str.to_string();

                                    tasks.push(Task { id, content })
                                }

                                Err(error) => {
                                    println!("ERROR[5]: {error}");
                                    exit(1);
                                }
                            }
                        }
                    }
                }

                Err(error) => {
                    println!("ERROR[4]: {error}");
                    exit(1);
                }
            }

            let task_id = match tasks.last() {
                Some(last_task) => last_task.id + 1,

                None => 1,
            };

            match OpenOptions::new().append(true).open(&taskfile_path) {
                Ok(mut taskfile) => match writeln!(taskfile, "{}", formatted_task) {
                    Ok(_) => {}

                    Err(error) => {
                        println!("ERROR[7]: {error}");
                        exit(1);
                    }
                },

                Err(error) => {
                    println!("ERROR[6]: {error}");
                    exit(1);
                }
            }
        }
        Command::View => {
            let mut viewed_tasks: Vec<Task> = Vec::new();
            let mut taskfile_path = temp_dir();
            taskfile_path.push("Taskfile");

            if !taskfile_path.exists() {
                match File::create(&taskfile_path) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("ERROR[0]: {error}");
                        exit(1);
                    }
                }
            }

            match read_to_string(&taskfile_path) {
                Ok(taskfile) => {
                    for line in taskfile.lines() {
                        if let Some((id_str, content_str)) = line.split_once(". ") {
                            match id_str.parse::<i32>() {
                                Ok(id) => {
                                    let content = content_str.to_string();

                                    // Finally
                                    viewed_tasks.push(Task { id, content })
                                }

                                Err(error) => {
                                    println!("ERROR[2]: {error}");
                                    exit(1);
                                }
                            }
                        }
                    }
                }

                Err(error) => {
                    println!("ERROR[1]: {error}");
                    exit(1);
                }
            }

            if !viewed_tasks.is_empty() {
                for viewed_task in viewed_tasks {
                    println!("{}. {}", viewed_task.id, viewed_task.content);
                }
            } else {
                println!("There are no tasks! Try adding one using the `add` command.");
            }
        }
        Command::Remove { id, all } => {
            let mut taskfile_path = temp_dir();
            taskfile_path.push("Taskfile");

            if !taskfile_path.exists() {
                match File::create(&taskfile_path) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("ERROR[8]: {error}");
                        exit(1);
                    }
                }
            }

            if !all && id.is_some() {
                if let Some(id) = id {
                    let mut tasks: Vec<Task> = Vec::new();

                    match read_to_string(&taskfile_path) {
                        Ok(taskfile) => {
                            for line in taskfile.lines() {
                                if let Some((id_str, content_str)) = line.split_once(". ") {
                                    match id_str.parse::<i32>() {
                                        Ok(id) => {
                                            let content = content_str.to_string();

                                            tasks.push(Task { id, content })
                                        }

                                        Err(error) => {
                                            println!("ERROR[10]: {error}");
                                            exit(1);
                                        }
                                    }
                                }
                            }
                        }

                        Err(error) => {
                            println!("ERROR[9]: {error}");
                            exit(1);
                        }
                    }

                    if !tasks.iter().any(|task| task.id == id) {
                        println!("Task not found!");
                        exit(1);
                    }

                    tasks.retain(|task| task.id != id);

                    match OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(&taskfile_path)
                    {
                        Ok(mut taskfile) => {
                            for task in &tasks {
                                let formatted_task = format!("{}. {}", task.id, task.content);
                                match writeln!(taskfile, "{}", formatted_task) {
                                    Ok(_) => {}
                                    Err(error) => {
                                        println!("ERROR[12]: {error}");
                                        exit(1);
                                    }
                                }
                            }
                        }

                        Err(error) => {
                            println!("ERROR[11]: {error}");
                            exit(1);
                        }
                    }
                    
                }
            } else if (!all && id.is_none()) || (all && id.is_some()) {
                println!("Either provide the --all flag or the <id> argument.");
                exit(1);
            } else {
                match OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&taskfile_path)
                {
                    Ok(_) => {}

                    Err(error) => {
                        println!("ERROR[13]: {error}");
                        exit(1);
                    }
                }
            }
        }
    }
}
