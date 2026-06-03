# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.10] - 2026-05-27

### Changed

- Copyright holder updated to Alexander Hurowitz in `LICENSE-*`

### Fixed

- CI: upgrade GitHub Actions for Node 24 runner compatibility

## [0.1.9] - 2026-05-07

### Changed

- Track `Cargo.lock` in the repo (publish with `--locked`)
- Dependency bumps: `rpassword`, `sysinfo`
- CI: extract Windows OpenSSL setup into a shared script
- README: MSRV badge → Rust 1.95

## [0.1.8] - 2026-04-27

### Changed

- MSRV **Rust 1.95**; **edition 2024**
- Refactor indexer streaming apply helpers
- Internal: int cast helpers; clippy allowances on bool-heavy opts structs

### Fixed

- CI: add `rustfmt` and `clippy` to the test workflow

## [0.1.7] - 2026-04-14

### Changed

- Dependency bumps only (no API or behavior changes expected)

## [0.1.6] - 2026-03-29

### Added

- **`tuning_for_path` consumers:** when `num_threads`, `drive_type`, and `use_parallel_walk` are all set on `NefaxOpts`, the pipeline skips drive/network probing and uses those values directly

### Changed

- Release workflow and dependency updates

## [0.1.5] - 2026-03-20

### Changed

- Dependency bumps (`clap`, `libc`, `sysinfo`, `toml`)
- Clippy and doc lint fixes across public and internal APIs

## [0.1.4] - 2026-03-01

### Changed

- Dependency updates
- Log no-changes diff at `info` instead of a lower level

## [0.1.3] - 2026-02-07

### Added

- **`validate_nefax`** — fail-early validation before passing a loaded snapshot as `existing` to `nefax_dir`
- Re-export **`Error`** and **`Result`** at the crate root

### Fixed

- Parallel walk (`jwalk`) now includes dotfiles, matching serial walk behavior
- Gitignore-style **`exclude`** patterns on `NefaxOpts`

### Changed

- Replace custom terminal colors with the `colored` crate; refine logger format

## [0.1.2] - 2026-02-04

Initial crates.io release.

### Added

- **`nefax_dir`** library entry point with optional streaming callback and `existing` snapshot diff
- CLI: index, `--dry-run`, Blake3 hashing, encryption (SQLCipher), exclude patterns, strict/paranoid modes
- Drive-adaptive pipeline (SSD / HDD / network tuning; `walkdir` vs `jwalk`)
- Streaming SQLite writes (WAL, batched inserts)
- `.nefaxer` on-disk schema and `.nefaxer.toml` CLI config

[0.1.10]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.9...v0.1.10
[0.1.9]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/Latka-Industries/nefaxer/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/Latka-Industries/nefaxer/releases/tag/v0.1.2
