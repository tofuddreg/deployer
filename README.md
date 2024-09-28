<div align="left">
    <img src="https://img.shields.io/badge/Rust-DDA484?logo=Rust&logoColor=white" />
    <!-- <img src="https://img.shields.io/badge/NodeJS-5FA04E?logo=Node.js&logoColor=white" />
    <img src="https://img.shields.io/badge/Spring_Boot-6DB33F?logo=Spring%20Boot&logoColor=white" />
    <img src="https://img.shields.io/badge/Go-00ADD8?logo=Go&logoColor=white" />
    <img src="https://img.shields.io/badge/Ruby-CC342D?logo=Ruby&logoColor=white" />
    <img src="https://img.shields.io/badge/Elixir-4B275F?logo=Elixir&logoColor=white" />
    <img src="https://img.shields.io/badge/Gleam-FF5CAA?logo=Gleam&logoColor=white" /> -->
</div>

# Deployer

Don't waste your time on deploying your projects! <br />
Deployer will check your repository's branch for new commits
and automatically pull your project into the specified directory.

<br/>

Note: Supports only projects from GitHub (will be fixed in the future)!
Note 2: In the config file, use only global paths to directories.

## Documentation

### Configuration

Firstly you wanna start with creating and modifying configuration file for Deployer. <br />
To generate new configuration file template, use the following command:

```Bash
deployer config /path/to/config
```

### Make it up and running

Once you have written the configuration file, you can run Deployer with this command:

```Bash
deployer run /path/to/config
```

Deployer will check your repository for new commits every 60 seconds.
