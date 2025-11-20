use std::io::{self, Write};

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
                println!("Unknown command");
            }
        }
    }

}