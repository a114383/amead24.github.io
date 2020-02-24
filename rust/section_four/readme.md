# Serde

## Cargo

This required some more digging into rustup vs. rustc vs. cargo.

I have a project that I created with `cargo new section_four`, and just wanted to create a simple `println!("args")` to start.  However `rustc src/main` doesn't compile the external libraries specified inside `cargo.toml`, while `cargo build --args` doesn't pass to main.

After reading through the [cargo documentation](https://doc.rust-lang.org/1.4.0/book/hello-cargo.html), turns out `cargo build` includes the necessary external libraries, however the binary is stored inside `./target/debug/<binary>`.

Conversely for quick testing you can also use: `cargo run -- <args>` to differentiate between cargo commands and those to be passed to the binary.

