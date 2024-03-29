# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- None

## [1.1.1] - 2022-03-11

### Changed
- Set `$BASH_ENV` to source the `nix-daemon.sh` file rather than the bespoke `$PATH` construction

## [1.1.0] - 2023-03-10

### Added
- New `install-cachix` command to install Cachix
- New `with-cachix` command to wrap Nix invocation with the Cachix `watch-exec [COMMAND]`
### Changed
- Rewrote command scripts in JavaScript for easier implementation and testing
- Migrated to [DeterminateSystems/nix-installer](https://github.com/DeterminateSystems/nix-installer)

## [1.0.0] - 2022-03-24

### Added

- Initial Release
- Ability to run the `nix/install` command to install the Nix package manager

[1.0.0]: https://github.com/ELD/nix-orb/releases/tag/v1.0.0
