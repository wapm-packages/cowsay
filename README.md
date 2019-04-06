# Cowsay

You can install cowsay by doing:

```shell
wapm install cowsay
```

*Note: This project is a fork of the original Rust implementation: [rust-cowsay](https://github.com/msmith491/rust-cowsay).*

## Running

```
wapm run cowsay "This is a test run"
 ___________________
< This is a test run >
 -------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```

## Included Cowfile

```
wapm run cowsay -f tux "This is a test run"
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

## Custom Cowfile

```shell
wapm run cowsay -f src/cows/elephant.cow "This is a test run"
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

# Building from Source

If you want to build the WebAssembly binary by yourself, you can do:

```shell
cargo +nightly build --release --target wasm32-unknown-wasi
```

This will create a new file located at `target/wasm32-unknown-wasi/release/cowsay.wasm`.

Once that file is created, you can upload it to wapm, or execute it with wasmer:

```shell
wasmer run  target/wasm32-unknown-wasi/release/cowsay.wasm "Hello World"
```

You can also build a native executable with

```shell
cargo build
```
