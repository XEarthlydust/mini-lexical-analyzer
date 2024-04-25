# A Mini C Lexical Analyzer

A mini C lexical analyzer implementation, written in Rust, for the purpose of practicing Rust
Program writing ~~and computer homework on writing compilation principles~~

## Analyze

- [x] Keyword
- [x] Identifier
- [X] Delimiter
- [X] Operator
- [X] Literal
  - [X] String
  - [X] Char
  - [X] Integer
    - [ ] Positive and negative signs
  - [X] Float
    - [ ] Positive and negative signs
- [ ] Pointer
- [X] Comments
  - [X] Single-Line
  - [X] Multi-Line

## Feature

- [X] Use `clap` to build CLI

## Usage

- Show help

    ```shell
    cargo run -- --help
    ```
- Analyze `*.c` file

    ```shell
    cargo run -- <FILE>
    ```