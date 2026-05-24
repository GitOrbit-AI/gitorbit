# Contributing to GitOrbit Protocol

First off, thank you for considering contributing to GitOrbit Protocol! It's explorers like you that make the intergalactic code network possible.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title** describing the issue
- **Steps to reproduce** the behavior
- **Expected behavior** vs actual behavior
- **Environment details** (OS, GitOrbit version, etc.)
- **Logs and error messages**

### Suggesting Features

Feature requests are welcome! Please:

- **Check existing issues** for similar suggestions
- **Describe the feature** in detail
- **Explain the use case** and why it's valuable
- **Consider implementation** approaches if possible

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. Ensure the test suite passes
4. Make sure your code follows the style guidelines
5. Issue the pull request!

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/gitorbit-protocol.git
cd gitorbit-protocol

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run
```

## Style Guidelines

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Write documentation for public APIs

### Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters
- Reference issues and pull requests liberally

### Documentation

- Keep README.md up to date
- Document all public APIs
- Include examples in documentation

## Project Structure

```
gitorbit-protocol/
├── src/
│   ├── identity/     # Cosmic DID and cryptography
│   ├── storage/      # Nebula storage integration
│   ├── network/      # libp2p networking
│   ├── agent/        # Agent spawning and management
│   ├── mcp/          # MCP server implementation
│   └── api/          # REST and GraphQL APIs
├── tests/            # Integration tests
├── docs/             # Documentation
└── examples/         # Example code
```

## Questions?

Feel free to open an issue or reach out on Discord!
