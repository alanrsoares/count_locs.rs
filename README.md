# Count LOCs

A blazing-fast command-line tool to recursively count lines of code in a directory, supporting custom glob patterns for file matching. Built with **Rust**, leveraging parallel processing for speed and efficiency.

## Features

- **Recursive directory traversal**
- **Glob pattern matching** for file extensions and paths
- **Parallel processing** for fast line counting
- **Breakdown of LOC** by glob pattern (when multiple patterns are used)
- Lightweight and easy to install as a global binary

## Installation

### Prerequisites

- Rust installed ([Install Rust](https://www.rust-lang.org/tools/install))

### Steps

1. Clone the repository:

   ```bash
   git clone git@github.com:alanrsoares/count_locs.rs.git
   cd count_locs
   ```

2. Build and install the binary:

   ```bash
   cargo install --path . --force
   ```

3. Ensure `~/.cargo/bin` is in your `PATH`:

   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

4. Verify installation:
   ```bash
   count_locs --help
   ```

## Usage

### Basic Command

```bash
count_locs <directory> <glob-patterns>...
```

### Examples

#### Count all Rust files in `./src`:

```bash
count_locs ./src "**/*.rs"
```

#### Count all TypeScript, TSX, and CSS files in a project:

```bash
count_locs ./ "**/*.ts" "**/*.tsx" "**/*.css"
```

#### Example Output:

```plaintext
Breakdown of Lines of Code by Glob:
  **/*.ts: 1200
  **/*.tsx: 800
  **/*.rs: 1500

Total lines of code: 3500
```

If only one glob pattern is used, the breakdown will be omitted:

```plaintext
Total lines of code: 1500
```

## Development

### Prerequisites

Ensure you have Rust installed. Use `rustup` to manage your Rust installation:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Install Locally for Development

```bash
cargo install --path . --force
```

## Makefile Support

A `Makefile` is included for convenience:

- **Build the project**: `make build`
- **Run tests**: `make test`
- **Install globally**: `make install`
- **Clean build artifacts**: `make clean`
- **Run the binary**: `make run`

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a feature branch.
3. Commit your changes.
4. Push to your branch.
5. Open a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

---

Happy coding! 🚀
