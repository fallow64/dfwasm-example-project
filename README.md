# A `dfwasm` example project

This is a Game of Life implementation using `dfwasm` to compile to DiamondFire.

It features allocation using [`wee_alloc`](https://docs.rs/wee_alloc/latest/wee_alloc/), and prints to DiamondFire using imports.

Here are the imports it expects (see [`src/df.rs`](/src/df.rs)):

- `env` `putc` - Puts a character to the console.
- `env` `wait` - Waits for a given number of milliseconds.
- `env` `millis` - Returns *some* type of current time in milliseconds.

## Compile to WASM

To compile the project to WASM, run the following command:

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Compile to DF

With the WASM binary from `/target/wasm32-unkown-unknown/release/dfwasm_example_project.wasm`, you can execute the `dfwasm` compiler:

```bash
dfwasm dfwasm_example_project.wasm --code-client # or --link for non-code-client
```