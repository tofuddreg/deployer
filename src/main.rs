use std::env;

mod generate_conf;
mod help;

const HELP_MSG: &str = "Use flag -h to see the documentation.";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", HELP_MSG);
        return;
    }

    if args.contains(&"-h".to_string()) {
        help::help();
        return;
    } else if args.contains(&"config".to_string()) {
        if args.len() < 3 {
            println!("{}", HELP_MSG);
            return;
        }
        let s: String = args[2].clone();
        let res = generate_conf::generate(&s);
        match res {
            Ok(path) => println!("path specified: {}", path),
            Err(e) => println!("{}", e),
        }
    }
    println!("exit success");
}
