# Rust Practice Repository

This repository is a collection of small Rust projects and examples created for learning and practice purposes.

## Getting Started

To get started with this repository, you'll need to have Rust and Cargo installed. You can find instructions on how to install them on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can clone this repository and build the project:

```bash
git clone <repository-url>
cd rust-practice
cargo build
```

To run a specific example, you can use the `cargo run --bin <binary-name>` command. For example, to run the TCP server, you would use the following command:

```bash
cargo run --bin server
```

## Project Structure

The project is organized into the following directories:

- `src/bin`: Contains the main application binaries. Each file in this directory is a separate executable.
- `src/utils`: Contains utility modules that are shared across the different binaries.
- `Notes`: Contains notes and other documentation.

## Binaries

The following binaries are available in the `src/bin` directory:

- `client`: A simple TCP client.
- `server`: A simple TCP server.
- `env_examples`: Examples of how to use environment variables.
- `simple_env`: A simple example of how to use environment variables.
- `rough`: A scratchpad for trying out new ideas.

## Utilities

The following utility modules are available in the `src/utils` directory:

- `array`: Functions for working with arrays.
- `checktypes`: Functions for checking the types of variables.
- `file_handling`: Functions for reading and writing files.

## .gitignore

This repository also includes a `.gitignore` file, which is a text file that tells Git which files or folders to ignore in a project. This is useful for preventing sensitive files, such as API keys or passwords, from being committed to the repository. It also helps to keep the repository clean by ignoring files that are generated during the build process, such as the `target` directory.
