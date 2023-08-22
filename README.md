# Intrust

Intrust *(**Int**erpretive **Rust**)* is a simple programming language and interpreter built in Rust. This repository contains the source code for the Intrust interpreter, which allows you to interact with the language through a Read-Eval-Print Loop (REPL).

## Getting Started

To use Intrust, you need to have Rust installed on your machine. You can install Rust by following the instructions on the official Rust website.

Once Rust is installed, you can clone this repository and navigate to the root directory of the project.

## Usage

To start the Intrust REPL, run the following command in your terminal:

```sh
cargo run
```

Optionally, binaries can be built and run directly via the following:
```sh
cargo build --release
cd ./target/release
./intrust
```

The REPL will greet you and display a prompt. You can enter Intrust code expressions at the prompt, and the interpreter will tokenize and process your input, displaying the parsed tokens and their types.

## Structure

The project is organized as follows:

- `main.rs`: The entry point of the Intrust interpreter. This file contains the main function that starts the REPL.
- `/libs/mod.rs`: The module file that defines the `lexer` module.
- `/libs/lexer.rs`: The lexer module that provides tokenization capabilities for the Intrust language.

## Contributing

Contributions to Intrust are welcome! If you'd like to contribute, feel free to fork this repository, make your changes, and submit a pull request. Please ensure that your code adheres to the project's coding style and standards.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Intrust was created as a learning project to explore programming language concepts and Rust. Special thanks to the Rust community and the creators of the libraries used in this project. The project is currently a massive WIP, and will be pieced together in my spare time.
