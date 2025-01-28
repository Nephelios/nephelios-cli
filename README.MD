# Nephelios CLI

A command-line interface tool for managing deployments on Nephelios, a Platform as a Service (PaaS) solution. This CLI makes it easy to deploy applications directly from GitHub repositories with simple commands.

## Features

- Easy deployment of applications to Nephelios from GitHub repositories
- Support for multiple application types (Node.js, Python, Rust)
- Interactive command-line interface with loading animations
- Comprehensive error handling and feedback

## Installation

### Prerequisites

- Rust and Cargo (1.70.0 or later)
- Git

### Local Development Setup

1. Clone the repository:

```bash
git clone https://github.com/Nephelios/nephelios-cli.git
cd nephelios-cli
```

2. Build the project:

```bash
cargo build
```

### Global Installation

To install the Nephelios CLI globally on your system:

```bash
cargo install --path .
```

After installation, you can run `nephelios-cli` from anywhere in your terminal.

## Usage

### During Development

While developing, use `cargo run` to test the CLI:

```bash
# Show help
cargo run -- --help

# Show help for create command
cargo run -- create --help

# Create a new deployment
cargo run -- create \
    --name my-awesome-app \
    --type nodejs \
    --github-url https://github.com/user/repo
```

### After Installation

Once installed globally:

```bash
# Show help
nephelios-cli --help

# Create a new deployment
nephelios-cli create \
    --name my-awesome-app \
    --type nodejs \
    --github-url https://github.com/user/repo
```

## Project Structure

```
nephelios-cli/
├── .github/
│   └── workflows/       # CI/CD workflows
├── src/
│   ├── main.rs         # Main CLI entry point
│   ├── commands/       # Command implementations
│   ├── types/          # Custom types and traits
│   └── utils/          # Utility functions
├── tests/              # Integration tests
├── .gitignore
├── Cargo.toml          # Project dependencies
├── Cargo.lock
└── README.md
```

## Contributing

We welcome contributions to Nephelios CLI! Here's how you can help:

1. Fork the repository
2. Create your feature branch:

```bash
git checkout -b feature/amazing-feature
```

3. Set up the development environment:

```bash
# Install dependencies
cargo build

# Run tests
cargo test
```

4. Make your changes and add tests if applicable
5. Commit your changes:

```bash
git commit -m 'Add some amazing feature'
```

6. Push to the branch:

```bash
git push origin feature/amazing-feature
```

7. Open a Pull Request

### Development Guidelines

- Follow Rust's standard formatting guidelines using `rustfmt`
- Add tests for new features
- Update documentation when adding or modifying features
- Use meaningful commit messages

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with logging
RUST_LOG=debug cargo test
```

## Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for CLI argument parsing
- Uses [reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- Uses [indicatif](https://github.com/console-rs/indicatif) for progress indicators

## Support

If you encounter any issues or have questions, please:

1. Check the [Issues](https://github.com/Nephelios/nephelios-cli/issues) page
2. Open a new issue if your problem isn't already reported

## Roadmap

Future planned features:

- [ ] Support for additional application types
- [ ] Deployment status monitoring
- [ ] Configuration file support
- [ ] Environment variable management
- [ ] Deployment logs viewing
- [ ] Integration with Nephelios dashboard
- [ ] Custom domain management
- [ ] Auto-scaling configuration
