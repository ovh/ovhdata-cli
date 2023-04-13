# OVHdata CLI

The ovhdata-cli allows you to use OVHcloud data products.
OVHcloud Data Integration (DI) is currently the only product availabe on the CLI. It allows you to extract your data from a source to a destination through schedulables workflows.

Crafted with Rust!
 
```bash
    ovhdata-cli di workflow run <workflow_id>
```
 
# Installation

```bash
    export SYSTEM_ARCHITECTURE=darwin # or linux or windows
    export DATA_PROCESSING_CLI_VERSION=$(curl -s https://api.github.com/repos/ovh/ovhdata-cli/releases/latest | grep "tag_name" | cut -d : -f 2,3 | tr -d \",\ )
    wget -O ovhdata-cli  https://github.com/ovh/ovhdata-cli/releases/download/$OVHDATA_CLI_VERSION/ovhdata-cli_$SYSTEM_ARCHITECTURE
    chmod u+x ovhdata-cli
```

In order to enable the completion on the CLI, please use the command:

```bash
    # generate the CLI completion script for your type of shell (bash, elvish, fish, powershell, zsh)
    ovhdata-cli completion bash > ~/ovhdata-cli-completion.sh
    # add to your rc file
    echo 'source ~/ovhdata-cli-completion.sh' > ~/.bashrc
    source ~/.bashrc
    # You may have to generate again the completion script as the CLI is updated and new commands may be available
```

You can now use tabulation to complete the CLI subcommands.

# Howto's

## Run
```
    Usage: ovhdata-cli [OPTIONS] <COMMAND>

    Commands:
    completion  Produces shell completion code for the specified shell
    config      Controls configuration of ovhdata-cli
    debug       Displays logs of a command executed by the cli
    di          Di (Data integration) product Subcommand
    login       Login into OVHcloud API on the current region
    logout      Removes OVHcloud API tokens on the current region
    me          Me from OVHcloud API
    help        Print this message or the help of the given subcommand(s)

    Options:
        --service-name <SERVICE_NAME>  OVHcloud service name to use
    -v, --verbose...                   Level of verbosity, can be used multiple times
        --json-log                     Log in json format rather than in plain text
        --no-color                     Remove colors from output
        --no-spinner                   Remove spinner from output
    -h, --help                         Print help
    -V, --version                      Print version
```

## Configure your CLI
The ovhdata-cli uses the OVHcloud API, hence it needs to be provided with credentials.
To create OVHcloud API credentials and set them it the CLI configuration, use the following command:

```bash
    ovhdata-cli login
    # this command should also print in the terminal the documentation on how to create credentials
    # if not run 'ovhdata-cli help login'
```

The data integration product requires that you own an OVHcloud Public Cloud project. For many subcommands of the ovhdata-cli, you will need to provide the CLI with a SERICE_NAME which is your Public Cloud project ID.
You can set once for all:
```bash
    ovhdata-cli config set-service-name
```
## Create a DI worflow
To create a new workflow, you will need a source and a destination based on the connectors of your choice. For instance, you can  pick/create a source wich is a S3 bucket and a destination which is a Postgresql DB.

You can find out what are the currently available connector by running:
```bash
    ovhdata-cli di source-connector list
    ovhdata-cli di destination-connector list
    # for more details on each connector
    ovhdata-cli di source-connector get <CONNECTOR_ID>
    ovhdata-cli di destination-connector get <CONNECTOR_ID>
```

> **_NOTE:_**  Most of the ovhdata-cli subcommands have interractive mode. Here for instance you, could have droped the CONNECTOR_ID, the CLI would have suggested the list of available connectors for you to select.

Source and destination are created in a similar way:
```bash
    # interractive if you omit the connector id or the parameters (required parameters are decribed in the connectors)
    ovhdata-cli di source create <NAME> --connector-id <CONNECTOR_ID> --parameter first_parameter_key=first_parameter_value second_parameter_key=second_parameter_value ...
```

Once you have a source and a destination, you can create a workflow:45
```bash
    # interractive if you omit a required configuration
    # here is the command with only the required information
    ovhdata-cli di workflow create --source-id <SOURCE_ID> --destination-id <DESTINATION_ID> --region <REGION> <NAME>
```

# Hacking
 
## Get the sources
 
```bash
    git clone https://github.com/ovh-ux/ovhdata-cli.git
    cd ovhdata-cli
```

## Compile

You have to install rust toolchain in order to compile ovhdata-cli. Please follow the [official documentation](https://www.rust-lang.org/tools/install)  

### Check code
```
cargo check
```

### Run tests
```
cargo test
```

### Compilation debug profile
Warning, the binary size maybe huge (don't forget that Rust produces large binaries), please use the release profile is you want to reduce it.  
```
cargo build
```
### Compilation release profile
By default, this profile use sharded libraries, it may not work on all linux distributions

#### Compilation linux with shared libraries
```
cargo build -r
```

#### Compilation linux statically linked
```
cargo build --target x86_64-unknown-linux-musl -r
```

#### Compilation Windows
```
cargo build --target x86_64-pc-windows-gnu -r
```

### Cross compile
You can generate the cli for many targets, you will find bellow the targets frequently used by OVHcloud.

| Name           | Target                     | Rustup command                              |
|----------------|----------------------------|---------------------------------------------|
| Windows        |  x86_64-pc-windows-gnu     | rustup target add x86_64-pc-windows-gnu     |
| Linux (static) |  x86_64-unknown-linux-musl | rustup target add x86_64-unknown-linux-musl |

Warning! Some targets additional packages (musl-tools on Ubuntu for example) 
 
 ## Contribute
You've developed a new cool feature? Fixed an annoying bug? We'd be happy
to hear from you!
 
Have a look in [CONTRIBUTING.md](https://github.com/ovh/ovhdata-cli/blob/master/CONTRIBUTING.md)
 
# Related links
 
 * Contribute: https://github.com/ovh/ovhdata-cli/blob/master/CONTRIBUTING.md
 * Report bugs: https://github.com/ovh/ovhdata-cli/issues
 * Get latest version: https://github.com/ovh/ovhdata-cli/releases
 
# License
 
Copyright 2023 OVH SAS
 
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at
 
    http://www.apache.org/licenses/LICENSE-2.0
 
Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.


