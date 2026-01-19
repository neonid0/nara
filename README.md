# Nara Language

[![CI](https://github.com/neonid0/nara/workflows/CI/badge.svg)](https://github.com/neonid0/nara/actions/workflows/ci.yml)
[![Release](https://github.com/neonid0/nara/workflows/Release/badge.svg)](https://github.com/neonid0/nara/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/neonid0/nara/branch/main/graph/badge.svg)](https://codecov.io/gh/neonid0/nara)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/nara.svg)](https://crates.io/crates/nara)

A modern programming language designed for building operating systems and systems software, focusing on simplicity, performance, and ease of use.

## Features

✨ **Current Features (v0.2.0)**
- 🔢 Numbers and floats
- 📝 Strings with interpolation (`f"Hello {name}!"`)
- ✅ Booleans and logical operators
- 🔀 Control flow (if/else, while, for)
- 📋 Lists and arrays
- 🎯 Functions with parameter binding
- 🏗️ Block expressions with scoping
- 🧵 String interning for memory efficiency
- 🔧 Built-in functions (print, len, range)

## Quick Start

### Installation

#### From crates.io
```bash
cargo install nara-cli
```

#### From source
```bash
git clone https://github.com/neonid0/nara.git
cd nara
cargo install --path crates/nara-cli
```

#### From release binaries
Download the latest release for your platform from [Releases](https://github.com/neonid0/nara/releases).

### Usage

```bash
# Run REPL
nara-cli

# Run a file
nara-cli script.nara

# Show help
nara-cli --help
```

### Example Code

**REPL Mode:**
```nara
-> val name = "Nara"
-> val greeting = f"Hello, {name}!"
-> print(greeting)
Hello, Nara!
```

**File Mode (current limitations):**
```nara
// Simple scripts work (keep on one line with semicolons):
val x = 10; val y = 20; print(x + y)

// Functions work:
fn double(x) { x + x }; val result = double(21); print(result)

// Loops work:
val nums = range(5); for n in nums { print(n) }
```

**⚠️ Known Limitations:**
- Complex expressions like `n * factorial(n - 1)` don't parse correctly (see [MISSING_FEATURES.md](MISSING_FEATURES.md))
- Multi-line files need proper formatting (best to use single line with semicolons)
- Comments (`//`) conflict with floor division operator
- No parenthesized expressions yet: `(a + b) * c` doesn't work

See [MISSING_FEATURES.md](MISSING_FEATURES.md) for complete list of limitations and workarounds.


## Development

### Prerequisites
- Rust 1.70+
- Git
- Make (optional)

### Setup
```bash
# Clone repository
git clone https://github.com/neonid0/nara.git
cd nara

# Checkout development branch
git checkout dev

# Run tests
make test

# Run linters
make clippy

# Format code
make fmt

# Run all checks
make all
```

### Development Workflow

We use a **main-dev** branching strategy:
- `main` - Production-ready code
- `dev` - Active development
- `feature/*` - New features
- `bugfix/*` - Bug fixes

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

## Documentation

- [CHANGELOG.md](CHANGELOG.md) - Version history
- [TEST_COVERAGE.md](TEST_COVERAGE.md) - Test coverage details  
- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- **[MISSING_FEATURES.md](MISSING_FEATURES.md) - Known limitations and planned features**
- [WORKFLOW_SETUP.md](WORKFLOW_SETUP.md) - CI/CD and development workflow
- [docs/](docs/) - Language documentation

## Roadmap

### v0.3.0 (Next Release) - Parser & Core Improvements
- [ ] **Fix operator precedence** (Critical - breaks recursive functions)
- [ ] **Parenthesized expressions** `(a + b) * c`
- [ ] **Proper comment handling** (fix conflict with `//` operator)
- [ ] Multi-line file support
- [ ] Return, break, continue statements
- [ ] Tuples `(a, b, c)`
- [ ] Method call syntax `obj.method()`
- [ ] Better error messages with line numbers

### v0.4.0 (Planned) - Type System
- [ ] Custom types (struct, enum)
- [ ] Pattern matching
- [ ] Type annotations (optional)
- [ ] Traits/Interfaces
- [ ] Module system
- [ ] Error handling (Result/Option types)
- [ ] Standard library expansion

### v0.5.0 (Planned) - Performance
- [ ] Generics
- [ ] AST optimization passes
- [ ] JIT compilation (experimental)
- [ ] Memory pooling

### v1.0.0 (Long-term) - Production Ready
- [ ] Self-hosting compiler
- [ ] Concurrency primitives
- [ ] FFI (Foreign Function Interface)
- [ ] Complete debugger
- [ ] LSP support
- [ ] Operating system development toolkit

**See [MISSING_FEATURES.md](MISSING_FEATURES.md) for detailed breakdown of limitations and workarounds.**


## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Coding standards
- Pull request process
- Release workflow

### Quick Contribution Guide

1. Fork the repository
2. Create a feature branch from `dev`
3. Make your changes
4. Run `make all` to ensure tests pass
5. Submit a PR to `dev` branch

## Community

- 🐛 [Report bugs](https://github.com/neonid0/nara/issues/new?template=bug_report.md)
- 💡 [Request features](https://github.com/neonid0/nara/issues/new?template=feature_request.md)
- 💬 [Discussions](https://github.com/neonid0/nara/discussions)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Built with ❤️ using Rust and inspired by modern programming language design.

---

## TODO

### Syntax
- Decide semicolon and return usage (should be optional or not?)
- nil or null or none
- Parentheses support in expressions
- Operation chaining

### Parser
- Operator precedence tables
- Better error messages with location info
- Incremental parsing support

### Features
- Method usage (anArray.map()?)
- Type system (type, enum, struct)
- Traits and implementations
- Access modifiers (public, private, protected)
- Static methods and properties
- Abstract types

### Performance
- AST optimization
- JIT compilation
- Memory pooling

---

**If you have any ideas or suggestions, please open an issue or PR!**

