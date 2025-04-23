# Contributing to Zaitun

Thank you for your interest in contributing to Zaitun! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and considerate of others when contributing to this project.

## How to Contribute

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development Setup

### Prerequisites
- Rust 1.50 or later
- CMake 3.10 or later
- LLVM 12 or later (for native code generation)

### Building from Source
```bash
git clone https://github.com/YourUsername/zaitun.git
cd zaitun
cargo build --release