use std::io::{self, Write};
use strsim::levenshtein;

fn main() {
    loop {

        print!("> ");
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
            _ => {
                suggestion(arr[0]   );
            }
        }
    }

}

fn suggestion(command : &str) {
    let commands = vec!["echo", "upper", "lower", "exit"];

    let mut current_command = "";
    let mut best_score = usize::MAX;

    for cmd in commands {
        let check = levenshtein(cmd, command);

        if check < best_score {
            current_command = cmd;
            best_score = check;
        }

    }

    if best_score >= 2 {
        println!("command not found ");
        println!("Did you mean : {}", current_command);
    }else {
            println!("command not found ");
    }

}