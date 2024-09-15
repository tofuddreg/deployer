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
