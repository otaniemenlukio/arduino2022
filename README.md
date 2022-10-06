# arduino2022

## setup Rust-compiler for Arduino boards with rustup
### *Doesn't (yet) work on Windows*
Make sure *rustup* is installed.
- Install nightly and rust-src compontents
```
$ rustup toolchain install nightly
$ rustup component add rust-src --toolchain nightly
```
- Install required third-party tools for your operating system from [here](https://book.avr-rust.com/002.1-installing-required-third-party-tools.html).
- Install the following crates
```
$ cargo +nightly install cargo-generate
$ cargo +nightly install ravedude
```

- NOTE: The 'libudev-dev' package (available on apt) might be required depending on your operating system.
- Initiate the template 

```
$ cargo generate --git https://github.com/Rahix/avr-hal-template.git
```
