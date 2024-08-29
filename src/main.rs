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

    if args.iter().any(|arg| arg == "-h") {
        help::help();
        return;
    } else if args.iter().any(|arg| arg == "config") {
        if args.len() < 3 {
            println!("{}", HELP_MSG);
            return;
        }
        generate_conf::generate(&args[2]).unwrap();
        println!("Path specified.");
    }
    println!("exit success");
}
