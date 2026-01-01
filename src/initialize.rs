use std::{io, process};
enum Init {
    Number(i32),
    Char(char),
}

pub fn home() -> i32 {
    loop {
        println!("Select para: ");

        println!("1: short \n2: medium \n3: large \nq: quit");
        
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 

        let input = input.trim();

        let choice = if let Ok(n) = input.parse::<i32>() {
            Init::Number(n)
        } else if input.len() == 1 {
            Init::Char(input.chars().next().unwrap())
        } else {
            println!("Invalid input!");
            continue;
        };

        match choice {
            Init::Number(n @ 1..=3) => {
                return n;
            },
            Init::Number(_) => {
                println!("Invalid number, Choose 1-3 \n");
            }
            Init::Char('q') => {
                println!("quitting!");
                process::exit(1);
            }
            _ => {
                println!("Invalid choice");
            }
        }      
    }
}
