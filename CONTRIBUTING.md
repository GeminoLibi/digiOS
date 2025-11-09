# Contributing to aiOS Rust

Thank you for your interest in contributing to aiOS! This document provides guidelines and instructions for contributing.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/aios-rust.git
   cd aios-rust
   ```

3. **Add upstream remote:**
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/aios-rust.git
   ```

4. **Create a branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Ensure all tests pass: `cargo test`

## Areas Needing Implementation

### High Priority
- [ ] Mouse/keyboard control (enigo, inputbot)
- [ ] Window management (Windows API, X11, Cocoa)
- [ ] Screenshot with cursor reticle
- [ ] File operations
- [ ] Process management

### Medium Priority
- [ ] Vision analysis
- [ ] OCR integration
- [ ] Task planning algorithms
- [ ] Event system implementation

### Low Priority
- [ ] Kernel-level operations
- [ ] Advanced learning system
- [ ] Performance optimizations

## Submitting Changes

1. **Ensure your code:**
   - Follows Rust best practices
   - Includes tests where appropriate
   - Is properly documented
   - Passes `cargo fmt` and `cargo clippy`

2. **Commit your changes:**
   ```bash
   git add .
   git commit -m "Description of your changes"
   ```

3. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create a Pull Request:**
   - Go to the original repository on GitHub
   - Click "New Pull Request"
   - Select your fork and branch
   - Fill out the PR template
   - Submit!

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn and grow

## Questions?

Open an issue for questions, bug reports, or feature requests.

Thank you for contributing to aiOS!

