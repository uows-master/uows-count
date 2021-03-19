// Copyright (C) 2021 Saadi Save, Varun Jain
// All rights reserved.
// Licensed under the GNU Affero General Public License
// (see LICENSE.md or <https://www.gnu.org/licenses/agpl-3.0.en.html>)
// All files in the project carrying such notice may not be copied, modified, or
// distributed except according to those terms.

/*!
# UOWS Count Server

The server responsible for counting votes the UOWS ecosystem.

## Installation
1. Go to releases
2. Download a binary for your OS and architecture
3. Rename it according to your own convinience
4. Either \
Execute the binary in place \
OR \
Add the binary/folder containing the binary to your `PATH`

## Usage
```text
UOWS Count Server <version>
Saadi Save, Varun Jain
Server responsible for counting votes in the UOWS ecosystem

USAGE:
    uows-count [FLAGS] -c <TOML FILE>

FLAGS:
    -h, --help       Prints help information
    -l               Enable logging. Log level is decided by the number of -l flags passed. It
                     ranges from 0 (off) to 3 (debug). Refer to the readme for more details.
    -r               Resets the datafile.
    -V, --version    Prints version information

OPTIONS:
    -c <TOML FILE>        The server configuration file, in toml
```

### Examples
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

### __[Important]__ Linux `ulimit` considerations
A low `ulimit` setting may cause the server to fail due to opening too many files. Therefore, it is recommended to run `ulimit -n 40000` in the terminal before starting the server.

## Configuration
```toml
# The ip address. Defaults to 127.0.0.1
address = "127.0.0.1"

# The listening port. Defaults to 5000
port = 8080

# The file with the candidate list. One candidate per line. No defaults.
# The server will panic if absent.
candidatesfile = "candidates"

# The file where the vote counts are stored in json format. Defaults to count.json.
datafile = "count.json"

# The file for the access key. No defaults. Server will panic if absent.
keyfile = "key"

# Enables SSL. Defaults to false.
secure = "true"

# The file for SSL certificate. No defaults. Server will panic if SSL is
# enabled, but this is absent.
cert = "private/cert.pem"

# The file for SSL private key. No defaults. Server will panic if SSL is
# enabled, but this is absent.
pkey = "private/key.pem"

# Sets log level. Ignored if -l flags are passed in the CLI.
log_level = 2
```

## Recommended Usage
Still under discussion
*/

#[macro_use]
extern crate rocket;

mod inits;
mod responses;
mod routes;
mod serve;
mod types;

#[cfg(test)]
mod tests;

#[launch]
async fn rocket() -> rocket::Rocket {
    serve::serve(&inits::parse_args().await).await
}
