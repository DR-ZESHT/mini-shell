use std::io::{self, Write};
use std::process::Command;
use strsim::levenshtein;
use colored::Colorize;
use std::path::Path;
use std::env;
use std::fs;
use whoami;

fn main() {

    loop {
        let hostname = whoami::fallible::hostname().expect("Unknown");
        let username = whoami::username();

        let input_path : String;
        if here() == username {
            input_path = "~".to_string();
        }else {
           input_path = here();
        }
        print!("[{}@{} {}]$ ", username, hostname.to_string(), input_path);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let arr: Vec<&str> = input.split_whitespace().collect();

        match arr[0] {
            "echo" => {
                println!("{}", arr[1..].join(" "));
            }
            "upper" => {
                print!("{}",arr[1..].join(" ").to_uppercase());
            }
            "lower" => {
                print!("{}",arr[1..].join(" ").to_lowercase());
            }
            "exit" => {
                break;
            }
            "cd" => {
                if arr.len() == 1 {
                    let path = format!("/home/{}",username );
                    cd(&path);
                }else {
                    cd(&arr[1..].join(" "));
                }
            }
            "ls" => {
                if arr.len() == 1 {
                    ls(".")
                }else {
                    ls(arr[1]);
                }
            }
            "pwd" => {
                println!("{}", pwd());

            }
            "clear" => {
                clear();
            }
            _ => {
                suggestion(arr[0]   );
            }
        }
    }

}

fn suggestion(command : &str) {
    let commands = vec!["echo", "upper", "lower", "exit", "cd","ls","pwd"];

    let mut current_command = "";
    let mut best_score = usize::MAX;

    for cmd in commands {
        let check = levenshtein(cmd, command);

        if check < best_score {
            current_command = cmd;
            best_score = check;
        }

    }

    if best_score <= 2 {
        println!("command not found ");
        println!("Did you mean : {}", current_command);
    }else {
            println!("command not found ");
    }

}

fn ls(path : &str) {

    let target_path = Path::new(path);

    let entries = fs::read_dir(target_path);

    match entries {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let filetype = entry.file_type().expect("failed to get filetype ");
                        let filename = entry.file_name().to_string_lossy().to_string();
                        let access = filename.chars().next().unwrap();

                        if access == '.' {
                            continue;
                        }else{
                            if filetype.is_dir() {
                                print!("{}  ", filename.blue());
                                io::stdout().flush().unwrap();
                            }else {
                                print!("{} ", filename);
                                io::stdout().flush().unwrap();
                            }


                        }

                    }
                    Err(e) => {
                        eprintln!("Warning: {}", e);
                    }
                }
            }

        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    println!("");
}
fn pwd() -> String {

    match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(e) => {println!("Error: {}", e);
            String::from("Unknown")
        }
    }
}

fn here() -> String {
    let  path = pwd();
    let get_array : Vec<String> = path.split('/').map(|s| s.to_string()).collect();
    return get_array[get_array.len() -1 ].clone();
}


fn cd(path : &str) {
    let root = Path::new(path);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("Error: {}", e);
    }
}
fn clear() {
    let _ = Command::new("clear").status();
}
