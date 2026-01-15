# Contributing to WhatsApp Desktop

First off, thank you for considering contributing to WhatsApp Desktop! It's people like you that make this project better.

## Code of Conduct

This project and everyone participating in it is governed by our commitment to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

When creating a bug report, include:
- **Clear title** describing the issue
- **Steps to reproduce** the behavior
- **Expected behavior** vs what actually happened
- **Screenshots** if applicable
- **System information** (Windows version, app version)

### Suggesting Features

Feature suggestions are welcome! Please:
- Check if the feature has already been suggested
- Provide a clear description of the feature
- Explain why this feature would be useful
- Consider how it might be implemented

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. Ensure the code builds without errors
4. Make sure your code follows the existing style
5. Write a clear PR description

## Development Setup

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)
- Windows 10/11 with Visual Studio Build Tools

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/whatsapp-desktop.git
cd whatsapp-desktop

# Install dependencies
npm install

# Run in development mode
npm run dev

# Build for production
npm run build
```

## Style Guidelines

### Rust Code
- Follow standard Rust formatting (`cargo fmt`)
- Use meaningful variable names
- Add comments for complex logic
- Handle errors properly (no unwrap in production paths)

### Commits
- Use clear, descriptive commit messages
- Keep commits focused on single changes
- Reference issues when applicable (`Fixes #123`)

## Questions?

Feel free to open an issue for any questions about contributing.

Thank you for your contribution! 🎉
