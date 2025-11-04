# Changelog

## Dev-stage-02

### Added
- Initial project structure with Tauri + Rust backend
- Modular crate architecture (lib-core, lib-git, lib-rpc, app-desktop)
- Frontend setup with TypeScript, Vite, and Rollup
- Basic Git repository operations foundation
- Cross-platform desktop application framework
- Development environment configuration

### Changed
- Project structure optimized for maintainability
- Build system configured for both development and production

### Added
- RPC

### Fixed
- Initial development environment setup issues
- Cross-platform compatibility configurations


---

## Development History

### Stage 02 (Current)
- **Branch**: `dev-stage-02`
- **Focus**: Core architecture and basic Git operations
- **Status**: In Progress
- **Key Components**:
  - Tauri application framework
  - Modular Rust crate structure
  - Frontend build system
  - Basic project configuration

### Stage 01 (Completed)
- **Branch**: `dev-stage-01`
- **Focus**: Initial project setup and architecture
- **Status**: Completed
- **Achievements**:
  - Project initialization
  - Basic folder structure
  - Development environment setup
  - Initial documentation

---

## Version History

### Pre-release Versions
- `v0.1.0-alpha.1`: Initial alpha release with basic structure

### Release Naming Convention
- **Alpha releases**: `v0.x.0-alpha.x` - Early development versions
- **Beta releases**: `v0.x.0-beta.x` - Feature-complete testing versions  
- **RC releases**: `v0.x.0-rc.x` - Release candidates
- **Stable releases**: `v0.x.0` - Production-ready versions
- **Branch Dev releases**: `dev-stage-0x/xx` - Early development versions

---

## Release Process

1. Update version in relevant files (`Cargo.toml`, `package.json`, `tauri.conf.json`)
2. Move "[Unreleased]" changes to a new version section
3. Add release date to the new section
4. Create and push a new git tag
5. Create a GitHub release with binary artifacts
6. Update documentation and announce the release
