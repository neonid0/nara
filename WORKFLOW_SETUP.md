# Nara Development Workflow Setup

This document describes the complete CI/CD setup and development workflow for the Nara project.

## Branching Strategy

### Main-Dev Workflow

```
main (production)
  ↑
  PR
  |
dev (integration)
  ↑
  PR
  |
feature/* or bugfix/*
```

- **main**: Production-ready code, protected branch
- **dev**: Active development and integration
- **feature/***: New features
- **bugfix/***: Bug fixes
- **release/***: Release preparation (automated)

## GitHub Actions Workflows

### 1. CI Workflow (`ci.yml`)

**Triggers:**
- Push to `main` or `dev`
- Pull requests to `main` or `dev`

**Jobs:**
- **Test Suite**: Runs on Linux, macOS, Windows
- **Rustfmt**: Code formatting check
- **Clippy**: Linting and best practices
- **Check**: Compilation check
- **Coverage**: Code coverage with Codecov

**Features:**
- Multi-platform testing
- Cargo caching for faster builds
- Comprehensive test coverage
- All warnings treated as errors

### 2. Release Workflow (`release.yml`)

**Triggers:**
- Git tags matching `v*.*.*` (e.g., v0.3.0)

**Jobs:**
- **Create Release**: Creates GitHub release
- **Build Release**: Builds binaries for all platforms
  - Linux x86_64 (glibc)
  - Linux x86_64 (musl)
  - macOS x86_64
  - macOS ARM64
  - Windows x86_64
- **Publish Crate**: Publishes to crates.io

**Artifacts:**
- Platform-specific binaries
- SHA256 checksums
- Source archives

### 3. Version Bump Workflow (`version-bump.yml`)

**Triggers:**
- Manual workflow dispatch

**Process:**
1. Select version type (patch/minor/major)
2. Updates version in Cargo.toml files
3. Updates CHANGELOG.md with release date
4. Creates PR from dev to main
5. Provides instructions for tagging

**Features:**
- Automated changelog updates
- Conventional commit integration
- PR template with checklist

### 4. Dependency Update Workflow (`dependency-update.yml`)

**Triggers:**
- Weekly schedule (Monday 00:00 UTC)
- Manual workflow dispatch

**Process:**
1. Updates Cargo.lock
2. Runs full test suite
3. Creates PR to dev branch

**Features:**
- Automated dependency management
- Test verification before PR
- Weekly security updates

### 5. Security Audit Workflow (`security.yml`)

**Triggers:**
- Push to main/dev affecting dependencies
- Pull requests
- Daily schedule
- Manual workflow dispatch

**Checks:**
- **cargo-audit**: Security vulnerabilities
- **cargo-deny**: License and ban checks

## Development Process

### Starting a New Feature

```bash
# 1. Update dev branch
git checkout dev
git pull origin dev

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Make changes and commit
# Follow conventional commits
git commit -m "feat(scope): description"

# 4. Run checks
make all

# 5. Push and create PR to dev
git push origin feature/my-feature
```

### Bug Fix Process

```bash
# 1. Create bugfix branch from dev
git checkout dev
git checkout -b bugfix/issue-123

# 2. Fix the bug
# Add tests to prevent regression

# 3. Commit with conventional commits
git commit -m "fix(scope): resolve issue #123"

# 4. Push and create PR to dev
git push origin bugfix/issue-123
```

### Release Process

#### Option 1: Using GitHub Actions (Recommended)

1. **Trigger Version Bump Workflow**
   - Go to Actions → Version Bump
   - Click "Run workflow"
   - Select version type (patch/minor/major)
   - Workflow creates PR to main

2. **Review and Merge PR**
   - Review generated PR
   - Verify CHANGELOG.md is correct
   - Ensure all CI checks pass
   - Merge to main

3. **Tag the Release**
   ```bash
   git checkout main
   git pull origin main
   git tag -a v0.3.0 -m "Release v0.3.0"
   git push origin v0.3.0
   ```

4. **Automated Release**
   - Release workflow triggers on tag
   - Builds binaries for all platforms
   - Creates GitHub release
   - Publishes to crates.io

#### Option 2: Manual Process

```bash
# 1. On dev branch
git checkout dev

# 2. Bump version
make bump-minor  # or bump-patch, bump-major

# 3. Update CHANGELOG.md manually

# 4. Commit and push
git commit -am "chore: bump version to 0.3.0"
git push origin dev

# 5. Create PR to main
# 6. After merge, tag the release
git checkout main
git pull origin main
git tag -a v0.3.0 -m "Release v0.3.0"
git push origin v0.3.0
```

## Conventional Commits

We use [Conventional Commits](https://www.conventionalcommits.org/) for clear, semantic commit messages.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code refactoring
- `perf`: Performance
- `test`: Tests
- `chore`: Maintenance
- `ci`: CI/CD changes

### Examples

```bash
feat(parser): add tuple support
fix(eval): correct operator precedence
docs: update README with examples
test: add tests for boolean operators
chore(deps): update dependencies
```

## Branch Protection Rules

### Main Branch
- Require pull request reviews
- Require status checks to pass
  - CI / Test Suite
  - CI / Rustfmt
  - CI / Clippy
- Require branches to be up to date
- Include administrators

### Dev Branch
- Require pull request reviews
- Require status checks to pass
- Allow force pushes for maintainers

## Required Secrets

Configure these in GitHub Settings → Secrets:

1. **CODECOV_TOKEN**: For code coverage reporting
2. **CARGO_TOKEN**: For publishing to crates.io

## Local Development Commands

```bash
# Run all checks (fmt, clippy, test, build)
make all

# Individual checks
make fmt      # Format code
make clippy   # Lint code
make test     # Run tests
make build    # Build debug binary
make release  # Build release binary

# Version management
make version      # Show current version
make bump-patch   # Bump patch version
make bump-minor   # Bump minor version
make bump-major   # Bump major version

# Release workflow
make release-workflow  # Run all checks + release build
make tag-release       # Create git tags
```

## Pull Request Checklist

Before submitting a PR:

- [ ] Code follows style guide
- [ ] All tests pass (`make test`)
- [ ] Clippy passes (`make clippy`)
- [ ] Code formatted (`make fmt`)
- [ ] New tests added for new features
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (for notable changes)
- [ ] Conventional commit messages used
- [ ] PR description filled out

## Code Review Process

1. **Automated Checks**: CI must pass
2. **Review**: At least one approval required
3. **Discussion**: Address all comments
4. **Merge**: Squash and merge to keep history clean

## Useful Links

- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Cargo Deny](https://embarkstudios.github.io/cargo-deny/)
- [Semantic Versioning](https://semver.org/)

## Troubleshooting

### CI Fails on Formatting
```bash
cargo fmt --all
git add -u
git commit --amend --no-edit
git push --force-with-lease
```

### CI Fails on Clippy
```bash
cargo clippy --all-targets --all-features -- -D warnings
# Fix warnings
git commit -am "fix: resolve clippy warnings"
```

### Release Workflow Fails
- Check CARGO_TOKEN secret is configured
- Verify version hasn't been published already
- Check binary builds succeed locally

---

For more details, see [CONTRIBUTING.md](CONTRIBUTING.md)
