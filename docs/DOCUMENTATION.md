# GitIT Documentation

This document provides comprehensive documentation for the GitIT project architecture, libraries, and development guidelines.

## Project Overview

GitIT is a modern Git client built with Rust and TypeScript, leveraging Tauri for cross-platform desktop application development. The project follows a modular architecture with clear separation between core functionality, Git operations, RPC communication, and the desktop application layer.

## Architecture

### Project Structure

The project is organized as a Rust workspace with multiple crates and a frontend TypeScript application:

```
GitIT/
├── crates/                 # Rust workspace members
│   ├── lib-core/          # Core shared functionality
│   ├── lib-git/           # Git operations library
│   ├── lib-rpc/           # RPC communication layer
│   └── app-desktop/       # Tauri desktop application
├── frontend/              # TypeScript frontend application
└── Configuration files
```

### Technology Stack

#### Backend (Rust)
- **Language**: Rust (Edition 2021/2024)
- **Framework**: Tauri v2
- **Async Runtime**: Tokio
- **Git Operations**: Git2
- **Serialization**: Serde
- **RPC Framework**: rpc-router
- **Build System**: Cargo

#### Frontend (TypeScript)
- **Language**: TypeScript
- **Build Tool**: Vite
- **Bundler**: Rollup
- **Styling**: PostCSS
- **Framework**: Native DOM (dom-native)

## Library Documentation

### lib-core

The core library provides shared functionality used across all other components.

**Location**: `crates/lib-core/`

**Dependencies**:
- `tauri` (v2): Cross-platform application framework
- `derive_more`: Derive macros for common traits
- `serde`: Serialization framework

**Purpose**:
- Common data structures and types
- Shared utilities and helpers
- Cross-crate abstractions
- Configuration management

**Usage**:
This library is intended to be used as a dependency by other crates in the workspace. It should not contain any application-specific logic, focusing instead on reusable components.

### lib-git

The Git library provides Git operations and repository management functionality.

**Location**: `crates/lib-git/`

**Dependencies**:
- `git2`: Git operations library
- `serde`/`serde_json`: Serialization support
- `derive_more`: Trait derivations

**Purpose**:
- Repository cloning and management
- Commit history analysis
- Branch operations
- File diff generation
- Git configuration handling

**Key Features**:
- Safe Git operations with error handling
- Async-compatible API design
- Serialization support for Git data structures
- Repository state management

**Usage**:
This library abstracts Git operations and provides a clean API for the application layer. It handles the complexity of Git2 library integration and provides type-safe operations.

### lib-rpc

The RPC library provides inter-process communication between the frontend and backend.

**Location**: `crates/lib-rpc/`

**Dependencies**:
- `rpc-router`: RPC framework for Tauri
- `serde`/`serde_json`: Message serialization
- `derive_more`: Trait implementations

**Purpose**:
- Define RPC command interfaces
- Handle frontend-backend communication
- Type-safe message passing
- Command routing and execution

**Key Features**:
- Type-safe RPC commands
- Automatic serialization/deserialization
- Error handling and propagation
- Command validation

**Usage**:
This library defines the contract between the frontend and backend, ensuring type safety and clear communication patterns.

### app-desktop

The desktop application crate contains the Tauri application implementation.

**Location**: `crates/app-desktop/`

**Dependencies**:
- `tauri` (v2): Desktop application framework
- `tauri-plugin-opener`: File opening capabilities
- `tauri-plugin-stronghold`: Secure storage
- `tokio`: Async runtime
- `serde`/`serde_json`: Configuration and data serialization
- `value-ext`: Value extension utilities

**Purpose**:
- Tauri application configuration
- Window management
- Menu and UI integration
- System integration
- Security configuration

**Key Features**:
- Cross-platform desktop application
- Native system integration
- Secure credential storage
- File system access management
- Auto-updater support

**Configuration**:
The application uses Tauri's configuration system defined in `tauri.conf.json` for window settings, security policies, and build configuration.

## Frontend Documentation

### Overview

The frontend is a TypeScript application that provides the user interface for GitIT. It communicates with the Rust backend through Tauri's IPC system.

**Location**: `frontend/`

### Technology Stack

- **TypeScript**: Type-safe JavaScript development
- **Vite**: Fast development server and build tool
- **Rollup**: Module bundler for production builds
- **PostCSS**: CSS processing and transformation
- **DOM Native**: Lightweight DOM manipulation library

### Build Configuration

- **Development Server**: Vite provides hot module replacement
- **Production Build**: Rollup creates optimized bundles
- **CSS Processing**: PostCSS handles styling with modern CSS features
- **Type Checking**: TypeScript compiler ensures type safety

### Integration with Backend

The frontend communicates with the Rust backend through:
- Tauri's IPC commands (defined in lib-rpc)
- Event system for real-time updates
- File system access through Tauri APIs
- Native dialogs and system integration

## Development Guidelines

### Code Organization

1. **Separation of Concerns**: Each library has a specific responsibility
2. **Type Safety**: Leverage Rust's and TypeScript's type systems
3. **Error Handling**: Comprehensive error handling at all layers
4. **Async Operations**: Use async/await patterns consistently
5. **Configuration**: Centralized configuration management

### Testing Strategy

- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test cross-crate interactions
- **Frontend Tests**: Test UI components and user interactions
- **End-to-End Tests**: Test complete user workflows

### Security Considerations

1. **Input Validation**: Validate all user inputs
2. **Safe Git Operations**: Use Git2 safely with proper error handling
3. **File System Access**: Restrict file access to necessary directories
4. **Credential Security**: Use secure storage for Git credentials
5. **Content Security Policy**: Configure CSP headers appropriately

### Performance Guidelines

1. **Async Operations**: Use Tokio for concurrent operations
2. **Resource Management**: Proper cleanup of Git repositories and file handles
3. **Frontend Optimization**: Minimize bundle size and optimize loading
4. **Memory Usage**: Be mindful of memory usage with large repositories
5. **Caching**: Implement appropriate caching strategies

## API Reference

### Core Types (lib-core)

The core library defines fundamental types used throughout the application. These include configuration structures, error types, and common data structures.

### Git Operations (lib-git)

The Git library provides operations such as:
- Repository initialization and cloning
- Commit history traversal
- Branch management
- Diff generation
- File status tracking

### RPC Commands (lib-rpc)

The RPC system supports commands for:
- Repository operations
- Git command execution
- Configuration management
- File system operations
- Application state queries

### Frontend API

The frontend provides:
- Repository browsing interface
- Commit history visualization
- Branch management UI
- File diff display
- Settings and configuration

## Configuration

### Workspace Configuration

The project uses Cargo workspace configuration defined in the root `Cargo.toml`:
- Shared dependencies are defined at workspace level
- Consistent versioning across crates
- Common build settings

### Application Configuration

Tauri application configuration is managed through:
- `tauri.conf.json`: Application settings, window configuration, security policies
- Environment-specific configurations
- User preferences and settings storage

### Frontend Configuration

Frontend build configuration:
- `vite.config.js`: Development server and build settings
- `rollup.config.js`: Production bundling configuration
- `tsconfig.json`: TypeScript compiler settings
- `postcss.config.js`: CSS processing configuration

## Build and Deployment

### Development Build

```bash
# Install dependencies
cargo build
npm install --prefix frontend

# Run development server
npm run dev --prefix frontend
```

### Production Build

```bash
# Build all components
cargo build --release
npm run build --prefix frontend

# Package desktop application
cargo tauri build
```

### Platform Support

The application supports:
- Windows (x64)
- macOS (x64, ARM64)
- Linux (x64)

## Troubleshooting

### Common Issues

1. **Git Repository Access**: Ensure proper file system permissions
2. **Build Failures**: Check Rust and Node.js versions
3. **Frontend-Backend Communication**: Verify RPC command definitions
4. **Platform-Specific Issues**: Consult Tauri platform guides

### Debug Information

Enable debug logging:
- Set `RUST_LOG=debug` environment variable
- Use browser developer tools for frontend debugging
- Check Tauri logs for application-level issues

## Contributing

For contribution guidelines, please refer to `CONTRIBUTING.md`. This includes:
- Code style guidelines
- Testing requirements
- Documentation standards
- Pull request process

## Resources

### External Documentation

- [Tauri Documentation](https://tauri.app/)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Git2 Documentation](https://docs.rs/git2/)
- [TypeScript Documentation](https://www.typescriptlang.org/)
