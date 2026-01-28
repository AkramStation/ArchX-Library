# Contributing to ArchX

First off, thank you for considering contributing to ArchX! It's people like you who make ArchX a great tool.

## Code of Conduct

ArchX follows the standard Rust Code of Conduct. Please be respectful and professional in all interactions.

## How Can I Contribute?

### Reporting Bugs
- Use the **GitHub Issue Tracker**.
- Provide a minimal reproducible example if possible.
- Include your hardware specs (CPU/GPU).

### Suggesting Enhancements
- Open a **Feature Request** issue.
- Explain the real-world use case and performance benefit.

### Pull Requests
1. Fork the repo.
2. Create a new branch: `git checkout -b feature/my-new-feature`.
3. Ensure all tests pass: `cargo test`.
4. Ensure code is formatted: `cargo fmt`.
5. Submit a PR against `main`.

## Development Principles

1. **Zero-Dependency**: Minimize external crates.
2. **Safety First**: Use `unsafe` only when SIMD intrinsics require it. Document the safety rationale.
3. **Performance counts**: Every PR should ideally include a benchmark comparison.

---
Designed with 🦀 by Codevora Studio.
