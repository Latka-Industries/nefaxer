# Nefaxer

[![Crates.io](https://img.shields.io/crates/v/nefaxer.svg)](https://crates.io/crates/nefaxer)
[![docs.rs](https://img.shields.io/docsrs/nefaxer)](https://docs.rs/nefaxer)
![Build](https://github.com/Latka-Industries/nefaxer/workflows/Build/badge.svg)
![Rust](https://img.shields.io/badge/rust-1.95-orange.svg)

> The Demon could sit in a box among air molecules that were moving at all different random speeds, and sort out the fast molecules from the slow ones.  
> — Koteks on the Nefastis Machine, _[The Crying of Lot 49][lot49]_

**Parallel directory indexing and change detection** — walk trees, store metadata in SQLite, diff against a previous snapshot (optional Blake3 hashes).

## Quick start

```bash
cargo install nefaxer
nefaxer /path/to/project

# diff only, no write
nefaxer --dry-run -l /path/to/project
```

Library: `cargo add nefaxer`. Releases: [Source Code][releases].

## Documentation

|                                                   |                                          |
| ------------------------------------------------- | ---------------------------------------- |
| **[Overview][nefax-site]**                        | Features, when to use standalone vs UBLX |
| [Install][nefax-install]                          | CLI and crate                            |
| [CLI][nefax-cli]                                  | Flags, dry-run, encryption               |
| [Configuration][nefax-config]                     | `.nefaxer.toml`                          |
| [Architecture][nefax-arch] · [Database][nefax-db] | Pipeline, schema                         |
| [Library][nefax-lib]                              | `nefax_dir`, streaming, tuning           |
| **[API (docs.rs)][docs-rs]**                      | Rust crate reference                     |

[UBLX][ublx] uses Nefaxer for snapshots, the Delta tab, and duplicate detection; enrich with [ZahirScan][zahirscan].

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).

[ublx]: https://github.com/Latka-Industries/UBLX
[zahirscan]: https://github.com/Latka-Industries/zahirscan
[nefax-site]: http://ublx.dev/nefaxer
[nefax-install]: http://ublx.dev/nefaxer/install
[nefax-cli]: http://ublx.dev/nefaxer/cli
[nefax-config]: http://ublx.dev/nefaxer/configuration
[nefax-arch]: http://ublx.dev/nefaxer/architecture
[nefax-db]: http://ublx.dev/nefaxer/database
[nefax-lib]: http://ublx.dev/nefaxer/library
[docs-rs]: https://docs.rs/nefaxer
[releases]: https://github.com/Latka-Industries/nefaxer/releases
[lot49]: https://bookshop.org/p/books/the-crying-of-lot-49-thomas-pynchon/e6265a50e173d7ec?ean=9780060913076&next=t
