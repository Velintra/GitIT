# GitIT

<img width="1920" height="991" alt="image" src="https://github.com/user-attachments/assets/bf458290-8f27-4441-961c-7a78f343a8fe" />


A modern, cross platform Git client built with Rust and Tauri v2, designed to provide an intuitive and powerful interface for Git operations.

## Features

- Cross-platform support for Windows, macOS, and Linux
- Modern web-based user interface with responsive design
- Comprehensive Git functionality including commits, branches, merges, and repository management
- High-performance Rust backend with memory safety guarantees
- Secure architecture with built-in security measures

## Architecture

GitIT follows a modular architecture with separate crates:

- `crates/app-desktop`: Tauri-based desktop application
- `crates/lib-core`: Core business logic and utilities
- `crates/lib-git`: Git operations and repository management
- `crates/lib-rpc`: RPC communication layer
- `frontend/`: Web-based user interface

## Technology Stack

### Backend
- **Rust**: Core backend language with memory safety
- **Tauri**: Cross-platform desktop application framework
- **Tokio**: Asynchronous runtime for concurrent operations
- **Git2**: Rust bindings for libgit2

### Frontend
- **TypeScript**: Type-safe frontend development
- **Vite**: Development server and build tool
- **Rollup**: Module bundler for production builds
- **PostCSS**: CSS processing and optimization

## Prerequisites

- Rust 1.70 or later
- Node.js 18.x or later
- Git 2.30 or later

## Documentation

- [Changelog](CHANGELOG.md) - Version history and changes
- [Security](SECURITY.md) - Security policies and vulnerability reporting
- [Development](DEVELOPMENT.md) - Detailed development setup and architecture

## Issue Reporting

Report bugs and request features through the GitHub issue tracker:
- Bug reports should include reproduction steps and environment details
- Feature requests should describe the use case and proposed implementation
