# Fast Delete (frm)

[![Latest Version](https://img.shields.io/crates/v/fast_delete.svg)](https://crates.io/crates/fast_delete)
[![Build Status](https://github.com/himasnhu-at/frm/actions/workflows/buildAndTest.yml/badge.svg)](https://github.com/himasnhu-at/frm/actions/workflows/buildAndTest.yml)
[![License](https://img.shields.io/badge/license-BSD%203--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

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
./target/release/frm <directory>
```

- `<directory>`: The path to the directory you want to delete.

## Example

```bash
./target/release/frm ../demo
```

This command will delete all files in `../demo`.

## Performance

Fast Delete has been benchmarked against the traditional `rm -rf` command. The results show that `frm` is slightly faster:

| Command | Time (s) |
| ------- | -------- |
| rm -rf  | 0m0.285s |
| frm     | 0m0.276s |

Additionally, here is a detailed summary of a deletion operation performed by `frm`:

```
Deletion Summary:
Files processed: 1687/1687
Directories removed: 9
Time taken: 267.32ms
Average speed: 6310.73 files/second
```

These results demonstrate that `frm` can handle large directories efficiently, making it a valuable tool for users who need to delete files and directories quickly.

## Documentation

To generate and open the documentation for Fast Delete, run the following command:

```bash
cargo doc --open --release
```

This will build the documentation and open it in your default web browser.

## Dependencies

- [walkdir](https://crates.io/crates/walkdir): For directory traversal.
- [rayon](https://crates.io/crates/rayon): For parallel processing.
- [indicatif](https://crates.io/crates/indicatif): For progress bar display.
- [num_cpus](https://crates.io/crates/num_cpus): To determine the number of available CPU cores.

## License

This project is licensed under the BSD 3-Clause License.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgments

- Inspired by the need for a faster file deletion tool.
