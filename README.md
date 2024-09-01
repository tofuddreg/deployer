# Deployer

Don't waste your time on deploying your projects! <br />
Deployer will check your repository's branch for new commits
and automatically pull your project into the specified directory.

<br/>

Note: Supports only projects from GitHub (will be fixed in the future)!

## Documentation

### Configuration

Firstly you wanna start with creating and modifying configuration file for Deployer. <br />
To generate new configuration file template, use the following command:

```Bash
deployer config /path/to/config
```

Note that you must specify **global** path to your desired cfg file location
(will be fixed in the future).

### Make it up and running

Once you have written the configuration file, you can run Deployer with this command:

```Bash
deployer run /path/to/config
```

Deployer will check your repository for new commits every sixty seconds. <br />
Note that you must specify **global** path to your cfg file (will be fixed in the future).
