# Lootbox random bot for World of Warships

## How to use

### Preparation

1. Have `rustc` and `cargo` installed on your machine.
2. Have `python` runtime installed on your machine.
3. Have `mongodb` installed on your machine or connectable via internet.

### Environment variables

The bot application use the dot-env file to configure its environment variables.

For the main application (bot binary and data loader):

```
MONGODB_CONN="mongodb://your/mongodb/connection"
WOWS_WEB_VERSION="<asset version of the official website | currently 8671650>"
RUST_LOG="<log level>"
CACHE_DIR="/path/to/binary/cache"
ASSET_FOLDER="/path/to/application/assets"
```

Note that the `ASSET_FOLDER` is just the `./asset` directory of this repository.

For bot's config, see [frontend server's README](./bin/python-bot/README.md).

### Load data

The `bin/wows-box-data-update` crate will download lootbox data from wg's website.

Run the following command:

```bash
cargo run --bin wows-box-data-update
```

**Note**: You have to update data manually if there's any update.

### Run bot

#### Bot backend server

No matter what frontend you want to use, you must keep the backend server running.

First, build the server:

```bash
cargo build --bin wows-rand-box --release
```

Then, copy it to somewhere else:

```bash
cp ./target/release/wows-rand-box /path/to/where/you/want
```

The runtime log directory will be created by the binary.

#### Frontend server

Currently the bot supports 3 target platforms, console, QQ and Discord. See [frontend server's README](./bin/python-bot/README.md) for more information.
