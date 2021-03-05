# uows-count

The server responsible for counting votes the UOWS ecosystem.

### Usage
    UOWS Count Server <version>
    Saadi Save, Varun Jain
    Server responsible for counting votes in the UOWS ecosystem

    USAGE:
        uows-count [FLAGS] -c <TOML FILE>

    FLAGS:
        -h, --help       Prints help information
        -l               Enable logging. Log level is decided by the number of -l flags passed. It
                        ranges from 0 (off) to 3 (debug). Refer to the readme for more details.
        -r               Resets the countfile.
        -V, --version    Prints version information

    OPTIONS:
        -c <TOML FILE>        The server configuration file, in toml

#### Examples
- `uows-count -c Config.toml` \
Only gives the config file. The data file is not reset. No logging.
- `uows-count -c Config.toml -r` \
The data file is reset. No logging
- `uows-count -c Config.toml -l` \
Critical errors are logged.
- `uows-count -c Config.toml -ll` \
All requests, config, etc. are logged
- `uows-count -c Config.toml -lll` \
Debug level logging. Should only be used during development.

### Configuration

### Recommended Usage
1. Recommended config:
```toml
address = "any address"
port = X443
keyfile = "file with key"
datafile = "file with the count"
candidatesfile = "file with the candidate list"
secure = "true"
cert = "File with ssl certificate"
pkey = "File with ssl private key"
```
2. Start the server with `uows-count -c Config.toml -r`
3. Ctrl-C immediately
4. Start the server with `uows-count -c Config.toml -l`
5. Ctrl-C after the voting time is over

[Still under progress]
