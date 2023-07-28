# Cowsay

You can install cowsay with:

```shell
wasmer run cowsay hello
```

*Note: This project is a fork of the original Rust implementation: [rust-cowsay](https://github.com/msmith491/rust-cowsay).*

## Running

```shell
$ wasmer run cowsay "This is a test run"
 ___________________
< This is a test run >
 -------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

Print help

```shell
$ wasmer run cowsay -h
cowsay v0.1.0
Syrus A. <syrus@wasmer.io>

USAGE:
    wasmer run cowsay [FLAGS] [OPTIONS] [MESSAGE]...

FLAGS:
    -b               Borg Cow
    -d               Dead Cow
    -g               Greedy Cow
    -h, --help       Prints help information
    -l, --list       List Cows
    -n               Disable word wrap
    -p               Paranoid Cow
        --random     Choose random cow
    -s               Stoned Cow
    -t               Tired Cow
    -V, --version    Prints version information
    -w               Wired Cow
    -y               Youthful Cow

OPTIONS:
    -f <COW>                  Which cow should say
    -e <EYE_STRING>           Custom Eyes
    -T <TONGUE_STRING>        Custom Tongue
    -W <WIDTH>                Max width of cow text bubble

ARGS:
    <MESSAGE>...    Message for cow to say
```


### Included Cowfile

```shell
$ wasmer run cowsay -f tux "This is a test run"
 ___________________
< This is a test run >
 -------------------
   \
    \
        .--.
       |o_o |
       |:_/ |
      //   \ \
     (|     | )
    /'\_   _/`\
    \___)=(___/
```

### Custom Cowfile

```shell
$ wasmer run cowsay -f src/cows/elephant.cow "This is a test run"
 ___________________
< This is a test run >
 -------------------
 \     /\  ___  /\
  \   // \/   \/ \\
     ((    O O    ))
      \\ /     \ //
       \/  | |  \/
        |  | |  |
        |  | |  |
        |   o   |
        | |   | |
        |m|   |m|
```

## Building from Source

First, you will need the WASI target installed in your Rust system:

```shell
rustup target add wasm32-wasi
```

Once WASI is available, you can build the WebAssembly binary by yourself with:

```shell
cargo build --release --target wasm32-wasi
```

This will create a new file located at `target/wasm32-wasi/release/cowsay.wasm`.

When the wasm file is created you can upload it to wasmer or execute it with wasmer:

```shell
wasmer publish
# OR
wasmer run . "Hello World"
```

You can also build a native executable with

```shell
cargo build
```
