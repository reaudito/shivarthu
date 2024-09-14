# Shivarthu

### Decentralized democracy with experts as leaders.

https://shivarthu.reaudito.com/#/

## Leptos/Rust Frontend

https://github.com/reaudito/shivarthu-client

## React Frontend (archieved)

https://github.com/amiyatulu/shivarthu_frontend

## Whitepaper

https://shivarthu.reaudito.com/paper/Shivarthu_whitepaper.pdf

## Technical Details

https://github.com/reaudito/shivarthu/blob/main/docs/Shivarthu.md


### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Podman Build

```
cargo build --release
podman build . -t=image1
podman run -d --name=container1 image1
```

### Tests
```
cargo test
```

### Embedded Docs

After you build the project, you can use the following command to explore its
parameters and subcommands:

```sh
./target/release/node-template -h
```

You can generate and view the [Rust
Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template
with this command:

```sh
cargo +nightly doc --open
```

### Single-Node Development Chain

The following command starts a single-node development chain that doesn't
persist state:

```sh
./target/release/node-template --dev
```

Use base path

```
-d, --base-path <PATH>
          Specify custom base path
```

```bash
./target/release/node-template  -d mychain-data --dev
```


To purge the development chain's state, run the following command:

```sh
./target/release/node-template purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```






