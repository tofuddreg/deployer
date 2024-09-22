use std::{env, io::ErrorKind};

use generate_conf::file_struct::Service;

mod generate_conf;
mod help;
mod macros;
mod run_deployer;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    arg_len!(args.len(), 2, macros::HELP_MSG);

    match args[1].as_str() {
        "--help" => help::help(),
        "config" => handle_generate(&args),
        "run" => handle_run(&args).await,
        "build" => handle_build(&args),
        _ => println!("{}", macros::HELP_MSG),
    }
}

fn handle_generate(args: &[String]) {
    arg_len!(args.len(), 3, macros::HELP_MSG);
    match generate_conf::generate(&args[2]) {
        Ok(()) => println!("Created successfully."),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => panic!("The configuration file alredy exists."),
            _ => panic!("An error occured while generating config file: {e}"),
        },
    }
}

async fn handle_run(args: &[String]) {
    arg_len!(args.len(), 3, macros::HELP_MSG);
    let mut path = String::from(&args[2]);
    generate_conf::validate_path(&mut path);
    run_deployer::run(&path).await;
}

use run_deployer::pull::build;

fn handle_build(_args: &[String]) {
    // arg_len!(args.len(), 3, macros::HELP_MSG);
    let mut services: Vec<Service> = Vec::new();
    services.push(Service::default());
    build::build(&services).expect("Valid services only");
}
