# Variables
TARGET = target/release/count_locs
BIN_PATH = ~/.cargo/bin/count_locs

# Build the project in release mode
build:
	cargo build --release

# Run tests
test:
	cargo test

# Install the binary globally
install: build
	cargo install --path . --force

# Clean the build artifacts
clean:
	cargo clean

# Run the binary (example usage)
run:
	$(TARGET) ./src "**/*.rs" "**/*.ts"

# Default target
default: build
