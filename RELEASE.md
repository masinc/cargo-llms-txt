# Release Procedure

This document outlines the steps for releasing a new version of `cargo-llms-txt`.

## Prerequisites

- Ensure you have push access to the main repository
- Ensure all CI checks are passing on the main branch
- Ensure all features are properly tested and documented
- Ensure `CARGO_REGISTRY_TOKEN` secret is configured in GitHub repository settings

## Release Checklist

### 1. Pre-release Checks

- [ ] All tests pass locally: `cargo test`
- [ ] Code is properly formatted: `cargo fmt --check`
- [ ] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] Integration tests pass: `cargo test integration_test --release`
- [ ] Documentation is up to date (README.md, CLAUDE.md)
- [ ] CHANGELOG.md is updated with new features and fixes (if exists)

### 2. Version Update

1. **Update version in Cargo.toml**
   ```toml
   [package]
   version = "X.Y.Z"  # Update to new version
   ```

2. **Update Cargo.lock**
   ```bash
   cargo check  # This updates Cargo.lock with new version
   ```

3. **Update version references in documentation** (if any)
   - Check README.md for hardcoded version references
   - Update installation instructions if needed

4. **Commit version bump**
   ```bash
   git add Cargo.toml Cargo.lock
   git commit -m "Bump version to X.Y.Z"
   git push origin main
   ```

### 3. Automated Release (Recommended)

The project uses GitHub Actions for automated releases. Simply push a version tag to trigger the complete release process:

1. **Create and push git tag**
   ```bash
   git tag -a vX.Y.Z -m "Release version X.Y.Z"
   git push origin vX.Y.Z
   ```

2. **Automated process will handle:**
   - Cross-platform binary builds (Linux, Windows, macOS including ARM64)
   - GitHub release creation with release notes
   - Binary asset uploads
   - Automated crates.io publishing
   - CI validation before publishing

3. **Monitor the release workflow:**
   - Go to the [GitHub Actions page](https://github.com/masinc/cargo-llms-txt/actions)
   - Watch the "Release" workflow progress
   - Verify all jobs complete successfully

### 4. Manual Release (Fallback)

If automated release fails, you can perform manual steps:

1. **Publish to crates.io**
   ```bash
   cargo login
   cargo publish --dry-run
   cargo publish
   ```

2. **Create GitHub Release manually**
   - Go to the [GitHub Releases page](https://github.com/masinc/cargo-llms-txt/releases)
   - Click "Create a new release"
   - Select the tag `vX.Y.Z` you created
   - Use the release notes template below
   - Upload binary assets manually if needed

### 5. Post-release Verification

- [ ] Verify the package appears on [crates.io](https://crates.io/crates/cargo-llms-txt)
- [ ] Test installation from crates.io: `cargo install cargo-llms-txt`
- [ ] Verify the binary works: `cargo llms-txt --version`
- [ ] Check that the GitHub release is properly created with binary assets
- [ ] Verify cross-platform binaries work on different operating systems
- [ ] Confirm release notes are accurate and complete

## Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version when you make incompatible API changes
- **MINOR** version when you add functionality in a backwards compatible manner
- **PATCH** version when you make backwards compatible bug fixes

## Release Notes Template

```markdown
## vX.Y.Z - YYYY-MM-DD

### Added
- New feature descriptions

### Changed
- Changed feature descriptions

### Fixed
- Bug fix descriptions

### Removed
- Removed feature descriptions (if any)

### Installation
```bash
cargo install cargo-llms-txt
```

### Usage
```bash
cargo llms-txt
```
```

## Rollback Procedure

If issues are discovered after release:

1. **Yank the problematic version from crates.io**
   ```bash
   cargo yank --vers X.Y.Z cargo-llms-txt
   ```

2. **Create a hotfix release with incremented patch version**

3. **Update GitHub release to mark it as "Pre-release" if severe issues exist**

## Automated Release Features

The GitHub Actions release workflow provides:

### ✅ Currently Automated
- **Cross-platform builds**: Linux (x86_64), Windows (x86_64), macOS (x86_64 + ARM64)
- **GitHub release creation**: Automatic release page with description
- **Binary asset uploads**: Pre-built binaries for all platforms
- **crates.io publishing**: Automated after successful tests
- **CI validation**: Full test suite runs before publishing
- **Release notes**: Auto-generated with commit history links

### 🔄 Future Improvements
Consider adding:
- Version bumping with tools like `cargo-release`
- Automatic changelog generation from conventional commits  
- Release candidate (RC) builds for testing
- Homebrew formula updates
- Documentation site updates

## GitHub Secrets Configuration

For automated releases to work, ensure these secrets are configured in GitHub repository settings:

1. **CARGO_REGISTRY_TOKEN**: Your crates.io API token
   - Get from [crates.io/me](https://crates.io/me)
   - Add to Settings > Secrets and variables > Actions

2. **GITHUB_TOKEN**: Automatically provided by GitHub Actions (no configuration needed)

## Troubleshooting

### Release Workflow Fails
- Check the Actions tab for detailed error logs
- Verify all CI checks pass on main branch before tagging
- Ensure `CARGO_REGISTRY_TOKEN` is correctly configured
- Check for crates.io naming conflicts or version issues

### Manual Release Steps
If automation fails, follow the manual release steps in section 4.

## Emergency Contacts

- Repository Owner: @masinc
- Backup Maintainer: (TBD)