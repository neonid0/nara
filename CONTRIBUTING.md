# Contributing to Nara

Thank you for your interest in contributing to Nara! This document provides guidelines and instructions for contributing.

## Development Workflow

We use a **main-dev** branching strategy:

- `main` - Production-ready code, always stable
- `dev` - Development branch, integration happens here
- `feature/*` - Feature branches
- `bugfix/*` - Bug fix branches
- `release/*` - Release preparation branches

### Branch Protection

- **main**: Protected, requires PR and passing CI
- **dev**: Integration branch, requires PR review

## Getting Started

1. **Fork the repository**
   ```bash
   git clone https://github.com/YOUR_USERNAME/nara.git
   cd nara
   ```

2. **Create a feature branch from dev**
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Write tests for new features
   - Update documentation
   - Follow the code style

4. **Test your changes**
   ```bash
   make all
   # or
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   cargo build
   ```

5. **Commit your changes**
   
   We follow [Conventional Commits](https://www.conventionalcommits.org/):
   
   ```
   <type>(<scope>): <subject>
   
   <body>
   
   <footer>
   ```
   
   **Types:**
   - `feat`: New feature
   - `fix`: Bug fix
   - `docs`: Documentation changes
   - `style`: Code style changes (formatting, etc.)
   - `refactor`: Code refactoring
   - `test`: Adding or updating tests
   - `chore`: Maintenance tasks
   - `perf`: Performance improvements
   - `ci`: CI/CD changes
   
   **Examples:**
   ```bash
   git commit -m "feat(parser): add support for tuples"
   git commit -m "fix(eval): correct boolean operator precedence"
   git commit -m "docs: update README with new examples"
   ```

6. **Push your branch**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Target the `dev` branch
   - Fill out the PR template
   - Link any related issues
   - Wait for CI to pass
   - Request review

## Pull Request Guidelines

### Before Submitting

- [ ] Code follows the project style
- [ ] All tests pass (`make test`)
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] Clippy passes (`make clippy`)
- [ ] Code formatted (`make fmt`)
- [ ] CHANGELOG.md updated (if applicable)

### PR Description

Include:
- **What** - What changes are being made
- **Why** - Why these changes are needed
- **How** - How the changes work
- **Testing** - How you tested the changes
- **Screenshots** - If UI changes

## Code Style

### Rust Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Fix all `cargo clippy` warnings
- Write documentation for public APIs
- Add examples in doc comments

### Testing

- Write unit tests for new functions
- Write integration tests for features
- Aim for >80% code coverage
- Test edge cases and error conditions

### Documentation

- Update README.md for user-facing changes
- Add doc comments for public APIs
- Update CHANGELOG.md for notable changes
- Add examples in `docs/` if needed

## Release Process

Releases are managed through GitHub Actions:

1. **Version Bump** (automated via workflow)
   - Go to Actions → Version Bump → Run workflow
   - Select patch/minor/major
   - This creates a PR to main

2. **Release PR Review**
   - Review the generated PR
   - Ensure CHANGELOG.md is correct
   - Merge when ready

3. **Tag and Release** (automated)
   - Merging to main triggers release workflow
   - Creates GitHub release
   - Builds binaries for all platforms
   - Publishes to crates.io

## Development Environment

### Required Tools

- Rust 1.70+ (use rustup)
- Git
- Make (optional, for convenience)

### Optional Tools

- `cargo-edit` - For version bumping
- `cargo-watch` - For auto-recompilation
- `cargo-tarpaulin` - For coverage reports

### IDE Setup

**VS Code:**
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.allFeatures": true
}
```

**IntelliJ/CLion:**
- Install Rust plugin
- Enable Clippy integration

## Testing

### Running Tests

```bash
# All tests
make test
# or
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Coverage
cargo tarpaulin --all-features
```

### Writing Tests

```rust
#[test]
fn test_feature() {
    // Arrange
    let input = "test";
    
    // Act
    let result = function(input);
    
    // Assert
    assert_eq!(result, expected);
}
```

## Debugging

### Logging

Use `RUST_LOG` environment variable:
```bash
RUST_LOG=debug cargo run
```

### Backtrace

```bash
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo test
```

## Common Tasks

### Add a new feature

```bash
git checkout dev
git pull origin dev
git checkout -b feature/my-feature
# ... make changes ...
make all
git commit -m "feat: add my feature"
git push origin feature/my-feature
# Create PR to dev
```

### Fix a bug

```bash
git checkout dev
git pull origin dev
git checkout -b bugfix/issue-123
# ... fix bug ...
make all
git commit -m "fix: resolve issue #123"
git push origin bugfix/issue-123
# Create PR to dev
```

### Update dependencies

Dependencies are automatically updated weekly via GitHub Actions.
To manually update:

```bash
cargo update
cargo test --all-features
```

## Getting Help

- **Issues**: Search existing issues or create a new one
- **Discussions**: Use GitHub Discussions for questions
- **Discord**: Join our Discord server (if available)

## Code of Conduct

Be respectful, inclusive, and constructive. We want Nara to have a welcoming community.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to Nara! 🎉
