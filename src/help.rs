struct Command<'a> {
    name: &'a str,
    description: &'a str,
}

/// Display list of available Deployer commands.
pub fn help() {
    let commands = [
        Command {
            name: "config <path>",
            description: "\tGenerate config and run Deployer with specified config.",
        },
        Command {
            name: "start <service>",
            description: "Starts a service.",
        },
        Command {
            name: "stop <service>",
            description: "\tStops a service.",
        },
        Command {
            name: "restart <service>",
            description: "Restarts a service.",
        },
        Command {
            name: "reload",
            description: "\t\tReload config file to apply new configuration.",
        },
        // Sort of dashboard where you'd see all services and their statuses
        // so you don't get lost which service is working and which is not.
        // Useful, especially for microservices.
        Command {
            name: "services status",
            description: "Display all specified services' statuses.",
        },
        Command {
            name: "overwrite",
            description: "\tCompletely deletes all .service files
            \t\t\t\t  (matched with services in the config file)
            \t\t\t\t  and replaces them with the new ones.",
        },
    ];

    println!("Available commands:");
    for c in commands {
        print!("\tdeployer {}", c.name);
        print!("\t{}\n", c.description);
    }
}
