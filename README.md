# Flashcards CLI
This is a simple application that I developed while trying to learn both Rust application development and VIM shortcuts.

The architecture is based loosely based off of [I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

The Flashcards CLI application is intended to read a YAML configuration file with Clue and Answer (C-A) pairs and then build a "deck" of flashcards to quiz the user on.

A "hand" of five cards will be randomly drawn from the deck and then individual cards from the hand have their Clue presented to the user as a question. Correct answers result in the presentation of the next card. Incorrect answers are given a second chance before the correct answer is displayed.

The difficulty of the generated deck of cards scales based on the level of the C-A pairs provided in the configuration file. After five correct answers, the deck is re-built with the next level of cards mixed in to increase difficulty and randomness.

The application can be terminated at any time with the "quit" command or Control-C keyboard shortcut.

## Building
After syncing the repo, the application can be built and executed like other Rust applications with:
```
cargo run
```
The flashcards_cli.exe can then be executed from the /target/debug directory.
The flashcards_cli.exe takes an optional file path argument for configuration. If no file path is provided, flashcards_cli
will attempt to use the default flashcards.yaml file.

