# ShareXtended

A simple program that deletes all images that are hosted on online image hosting by sending POST requests to all DeletionURL's.

## Usage

```ps1
./sharextended.exe --help
```

### TODO
- Unwrap and expect should be replaced with proper matching or flow (make good use of Option/Result).
- Add GitHub action to build and release binaries (Windows only, since ShareX is Windows only). See [actions/rust-release](https://github.com/marketplace/actions/rust-release) or [actions/rust-release-binary](https://github.com/marketplace/actions/rust-release-binary)
- Implement concurrent or parallel requests to become "Blazingly Fast™ 🔥" (see https://stackoverflow.com/a/51047786)

## Motivation

### Why Rust?
I wanted to try out the most loved language for myself.
While it's a bit hard to get started with due to these new concepts, like ownership, borrowing, and lifetimes, I really like it.
It's a very powerful language with great error messages and many useful crates. I'm looking forward to learn more about it.

### Code
Watching video's from [No Boilerplate](https://www.youtube.com/c/NoBoilerplate) motivated me to try out Rust. On advise about your Rust toolkit, he suggested the following Clippy lints:

```ps1
cargo clippy --fix -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used
```

For now I omit the `clippy::unwrap_used`. It's not great to use unwrap as often as I do, but I use it for getting things working. I'm sure I'll get better at making use of `Option` and `Result` in the future.

I tried to store the static strings in the `Cargo.toml`, but I figured it's not meant to live outside the code. Also tried to resolve `%USERPROFILE%` in the code but solved it with the `directories` crate.

I used https://transform.tools/json-to-rust-serde as a reference for the JSON structure.
