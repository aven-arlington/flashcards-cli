# Flashcards CLI
This is a simple application that I developed while trying to learn both Rust application development and VIM shortcuts.

The architecture is based loosely based off of [I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

The Flashcards CLI application is intended to read a file with "Clue" and "Answer" pairs from a configuration file. Then looping over random flashcard entries as correct answers are entered. If an incorrect answer, a second chance to answer the question will be provided. After a 2nd incorrect answer, the correct answer will be provided.


## Building
After syncing the repo, the application can be built and executed like other Rust applications with:
```
cargo run
```
The flashcards_cli.exe can then be executed from the /target/debug directory.
The flashcards_cli.exe takes an optional file path argument for configuration. If no file path is provided, flashcards_cli
will attempt to use the default flashcards.yaml file.

## Debugging
To see debug information on stdout, create the RUST_LOG environment variable. On Windows Powershell:
```
$env:RUST_LOG = "debug"
```
You can replace "debug" with the desired logging level: info, warn, debug, or error (the default).
