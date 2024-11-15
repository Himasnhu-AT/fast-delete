# Fast Delete

Fast Delete is a Rust-based command-line tool designed to be a faster alternative to the traditional `rm -rf` command. It leverages multithreading to efficiently delete files and directories.

## Features

- **Multithreaded Deletion**: Utilizes a thread pool to delete files concurrently, significantly speeding up the deletion process.
- **Progress Tracking**: Displays a progress bar to track the deletion process in real-time.
- **Directory Cleanup**: Removes empty directories after file deletion.

## Installation

To build and install Fast Delete, ensure you have Rust and Cargo installed, then run:

```bash
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## Usage

```bash
cargo run --release -- <directory>
```

- `<directory>`: The path to the directory you want to delete.

## Example

```bash
cargo run --release -- ../demo
```

This command will delete all files in `../demo`.

## Dependencies

- [walkdir](https://crates.io/crates/walkdir): For directory traversal.
- [rayon](https://crates.io/crates/rayon): For parallel processing.
- [indicatif](https://crates.io/crates/indicatif): For progress bar display.
- [num_cpus](https://crates.io/crates/num_cpus): To determine the number of available CPU cores.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgments

- Inspired by the need for a faster file deletion tool.
