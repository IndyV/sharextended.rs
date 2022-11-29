# ShareXtended

A Rusty program that provides ShareX utility commands.

## Usage

```ps1
Usage: sharextended.exe [COMMAND]

Commands:
  purge-online  Delete all screenshots that have been uploaded to Imgur
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```
> If `[COMMAND]` is not specified the program will run in interactive mode.


## Motivation

I started this project a while ago after an online friend asked me how he could delete all his images hosted on Imgur. While a simple script was enough to help him, I wanted to make the project as accessible as possible by making it a standalone executable. I made that project with TS/Node.js and was planning to package it. I tried to use projects like [PKG](https://github.com/vercel/pkg), but I kept having issues trying to package it. It wasn't able to package the [Open](https://www.npmjs.com/package/open) dependency and would have to be downloaded alongside the package.

### Why Rust?

Recently I just wanted to try out [the most loved language](https://survey.stackoverflow.co/2022/#technology-most-loved-dreaded-and-wanted) for myself and was looking for a project to get started with. This project seemed like the perfect fit since it also allows for easy distribution with cross-platform compilation.
So I decided to try Rust, and it worked like a charm.

Yes, I know this project could have also been done in many other (easier) ways, like:
- C#, powerful and also matching the language of the ShareX project.
- Or even simpler as a Powershell or Python script probably

But this project wasn't for any business related task and I wanted to try out Rust. So I did.

It's a bit hard to get started with due to these new concepts, like ownership, borrowing, and lifetimes, but I really like it.
It's a very powerful language with great error messages and many useful crates. I'm looking forward to learn more about it.


### Code
Watching video's from [No Boilerplate](https://www.youtube.com/c/NoBoilerplate) motivated me to try out Rust. On advise about your Rust toolkit, he suggested the following Clippy lints:

```ps1
cargo clippy --fix -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used
```

For now I omit the `clippy::unwrap_used`. It's not great to use unwrap as often as I do, but I use it for getting things working. I'm sure I'll get better at making use of `Option` and `Result` in the future.

I tried to store the static strings in the `Cargo.toml`, but I figured it's not meant to live outside the code. Also tried to resolve `%USERPROFILE%` in the code but solved it with the `directories` crate.

I used https://transform.tools/json-to-rust-serde as a reference for the JSON structure.

#### Crates used
- `eyre`: For simple error reporting
- `tokyo` & `futures`: For writing async Rust
- `serde` & `serde_json`: For (JSON) Serialization/Deserialization
- `clap`: For CLI parsing
- `reqwest`: For HTTP requests
- `ansi_term`: For colored output
- `chrono`: For time parsing and formatting
- `indicatif`: For progress bars
- `dialoguer`: For interactive prompts
- `tinyfiledialogs`: For file dialogs
- `open`: For opening files in the default application
- `lazy_static`: For static variables that require evaluation at runtime
- `console`: For console input/output
- `directories`: For getting the user's home directory
