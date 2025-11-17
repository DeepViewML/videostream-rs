# Contributing to videostream-rs

Thank you for your interest in contributing to videostream-rs! This project provides
Rust bindings for the VideoStream Library and is a key component of the EdgeFirst
Perception stack, advancing edge AI and computer vision capabilities on resource-
constrained embedded devices.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Before You Start](#before-you-start)
- [Development Setup](#development-setup)
- [Contribution Process](#contribution-process)
- [Code Style](#code-style)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)
- [License](#license)

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) before contributing. We are
committed to providing a welcoming and inclusive environment for all contributors.

## Ways to Contribute

We welcome many types of contributions:

- **Code:** New features, bug fixes, performance improvements, refactoring
- **Documentation:** Improvements to docs, examples, tutorials, API documentation
- **Testing:** Bug reports, test coverage improvements, hardware platform validation
- **Community:** Answer questions in discussions, write blog posts, speak at meetups
- **Integration:** Examples integrating with EdgeFirst Studio or other systems

## Before You Start

To ensure your contribution can be reviewed and merged efficiently:

1. **Check Existing Work:**
   - Browse [existing issues](https://github.com/EdgeFirstAI/videostream-rs/issues) and
     [pull requests](https://github.com/EdgeFirstAI/videostream-rs/pulls)
   - Avoid duplicating effort by searching first

2. **Discuss Significant Changes:**
   - For major features or architectural changes, open an issue for discussion first
   - This helps align your work with project goals and avoid wasted effort

3. **Review the Roadmap:**
   - Check the project [roadmap](https://github.com/EdgeFirstAI/videostream-rs/issues?q=is%3Aissue+is%3Aopen+label%3Aroadmap)
     to understand our direction
   - See where your contribution fits in the bigger picture

4. **Consider EdgeFirst Integration:**
   - Think about how changes might affect EdgeFirst Studio integration
   - Consider impact on hardware platforms (Maivin, Raivin, etc.)
   - Ensure backward compatibility when possible

## Development Setup

### Prerequisites

**Required:**
- **Rust:** 1.70 or newer (stable channel)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **libvideostream:** Native VideoStream Library (version 1.3.9 or newer)
  - Contact support@au-zone.com for access to libvideostream
  - Or see build instructions in the [documentation](https://doc.edgefirst.ai)
- **System Dependencies:**
  - Linux development environment (Ubuntu 20.04+ recommended)
  - C compiler (gcc or clang)
  - pkg-config

**Optional (for full feature development):**
- **Hardware Platforms:** Maivin or Raivin device for hardware-accelerated features
- **EdgeFirst Studio Account:** Free tier available at https://edgefirst.studio
  for testing Studio integration

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/EdgeFirstAI/videostream-rs.git
cd videostream-rs

# Build the project
cargo build

# Run tests (requires libvideostream)
cargo test

# Build documentation
cargo doc --open
```

### Development Tools

Install recommended development tools:

```bash
# Formatter (already configured in rustfmt.toml)
rustup component add rustfmt

# Linter
rustup component add clippy

# Security auditing
cargo install cargo-audit

# Test coverage
cargo install cargo-llvm-cov
```

## Contribution Process

### 1. Fork and Clone

```bash
# Fork on GitHub, then clone your fork
git clone https://github.com/YOUR-USERNAME/videostream-rs.git
cd videostream-rs

# Add upstream remote
git remote add upstream https://github.com/EdgeFirstAI/videostream-rs.git
```

### 2. Create Feature Branch

Use a descriptive branch name:

```bash
# For new features
git checkout -b feature/descriptive-name

# For bug fixes
git checkout -b bugfix/issue-number-description

# For documentation
git checkout -b docs/what-you-are-documenting
```

### 3. Make Changes

- **Write Clean Code:** Follow Rust idioms and project patterns
- **Add Tests:** Every new feature or bug fix should include tests
- **Update Documentation:** Keep docs in sync with code changes
- **Commit Frequently:** Make logical, atomic commits

### 4. Follow Code Style

Before committing:

```bash
# Format code
cargo fmt

# Check for lints
cargo clippy --all-targets --all-features

# Run tests
cargo test

# Check documentation
cargo doc --no-deps
```

### 5. Submit Pull Request

**Prepare Your PR:**
```bash
# Update your branch with latest upstream
git fetch upstream
git rebase upstream/main

# Push to your fork
git push origin your-branch-name
```

**Create Pull Request:**
- Go to https://github.com/EdgeFirstAI/videostream-rs/pulls
- Click "New Pull Request"
- Select your fork and branch
- Fill out the PR template:
  - **Title:** Clear, concise description (e.g., "Add support for YUV420 format")
  - **Description:**
    - What problem does this solve?
    - What changes did you make?
    - How did you test it?
    - Any breaking changes?
  - **Link Related Issues:** Use "Fixes #123" or "Relates to #456"

**PR Review Process:**
- Maintainers will review your PR (typically within 1 week)
- Address any feedback by pushing new commits
- Once approved, a maintainer will merge your PR
- Your commits will be preserved with author attribution

## Code Style

### Rust Formatting

We use `rustfmt` with the configuration in `rustfmt.toml`:

```bash
# Format all code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

**Key formatting rules:**
- Maximum line width: 100 characters
- Use 4 spaces for indentation (no tabs)
- Follow Rust naming conventions (snake_case, CamelCase, etc.)

### Clippy Lints

We enforce Clippy lints as warnings:

```bash
# Run clippy
cargo clippy --all-targets --all-features

# Fix auto-fixable lints
cargo clippy --fix
```

**Required lint levels:**
- No warnings allowed in CI
- Prefer idiomatic Rust over clever code
- Use `#[allow(...)]` sparingly and with justification

### Code Organization

- **Public API:** Well-documented, stable, follows Rust API guidelines
- **Internal Code:** Can be more flexible, but still clear and maintainable
- **Unsafe Code:** Must have safety comments explaining why it's safe
- **Error Handling:** Use `Result<T, E>` with meaningful error types
- **FFI Boundaries:** Carefully validate all data crossing the FFI boundary

## Testing Requirements

### Unit Tests

- **Coverage:** Minimum 70% line coverage for new code
- **Location:** Tests should be in the same file as the code (using `#[cfg(test)]`)
  or in a `tests/` subdirectory for integration tests
- **Naming:** Use descriptive test names: `test_client_reconnects_after_disconnect`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_creation() {
        // Test implementation
    }
}
```

### Integration Tests

For features requiring the native library:

```rust
// tests/integration_test.rs
#[test]
#[ignore] // Ignored by default, requires libvideostream
fn test_client_host_communication() {
    // Test implementation
}
```

Run with:
```bash
cargo test -- --ignored
```

### Hardware Platform Tests

If you have access to Maivin or Raivin hardware:

```bash
# Run tests on target hardware
cargo test --release --target aarch64-unknown-linux-gnu
```

Document any hardware-specific test requirements in your PR.

### Test Coverage

Check coverage locally:

```bash
# Generate coverage report
cargo llvm-cov --html

# Open report
open target/llvm-cov/html/index.html
```

## Documentation

All public APIs must be documented with rustdoc comments.

### API Documentation

```rust
/// Creates a new VideoStream client.
///
/// # Arguments
///
/// * `path` - The UNIX domain socket path for the VideoStream host
/// * `reconnect` - Whether to automatically reconnect on disconnection
///
/// # Errors
///
/// Returns an error if:
/// - The socket path is invalid
/// - Connection to the host fails (and `reconnect` is false)
///
/// # Example
///
/// ```no_run
/// use videostream::Client;
///
/// let client = Client::new("/tmp/videostream.sock", true)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn new(path: &str, reconnect: bool) -> Result<Self, Box<dyn Error>> {
    // Implementation
}
```

### Module Documentation

Each module should have a module-level doc comment:

```rust
//! Client module for subscribing to VideoStream frames.
//!
//! This module provides the [`Client`] type for receiving video frames
//! from a [`Host`](crate::host::Host) over UNIX domain sockets with
//! zero-copy shared memory.
```

### Examples

Provide working examples in `examples/` directory:

```bash
# Run an example
cargo run --example basic_client
```

## License

By contributing to videostream-rs, you agree that your contributions will be
licensed under the [Apache License 2.0](LICENSE).

**Important:**
- No Contributor License Agreement (CLA) is required
- You retain copyright to your contributions
- Your code will be distributed under Apache-2.0 along with the rest of the project

## Getting Help

If you have questions or need assistance:

- **GitHub Discussions:** Ask questions and discuss ideas
- **Issues:** Report bugs or request features
- **Email:** For private inquiries, contact support@au-zone.com
- **Documentation:** Check https://doc.edgefirst.ai for detailed guides

## Recognition

We value our contributors! Your contributions will be recognized:

- Author attribution in git history
- Mention in release notes (for significant contributions)
- Listed in CONTRIBUTORS.md (coming soon)
- Invited to join the EdgeFirst developer community

Thank you for helping make videostream-rs better! 🎉

---

**Last Updated:** 2025-11-14
**Version:** 1.0
