# Mirl Extensions Conversion (0.0.0-alpha)

> This is a sub-crate of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) ([crates.io](https://crates.io/crates/mirl_extensions))

### Mextco - Convert between data types (or try to)

[![Version](https://img.shields.io/crates/v/mirl_extensions_conversion.svg)](https://crates.io/crates/mirl_extensions_conversion)
[![Documentation](https://docs.rs/mirl_extensions_conversion/badge.svg)](https://docs.rs/mirl_extensions_conversion)
[![dependency status](https://deps.rs/repo/github/Miner3D-Gamer/mirl_extensions_conversion/status.svg)](https://deps.rs/repo/github/Miner3D-Gamer/mirl_extensions_conversion)
[![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.99–nightly-blue)](https://github.com/rust-lang/rust/releases/tag/1.99.0)
[![Changelog](https://img.shields.io/badge/CHANGELOG.md--555.svg)](https://github.com/Miner3D-Gamer/mirl_extensions_conversion/blob/main/CHANGELOG.md)

<details>
<summary>Flags</summary>

### Default

**Core**

- `std` (Default)
  ~~- `c_compatible`~~ Only traits and impls are defined
- `all`

**Custom**
TODO

</details>

### Entry Points

- `use mirl_extensions_conversion::*;`

### Purpose

Patch the holes inside of `core::conversion::From/TryFrom` which even when only taking numbers into consideration, has holes include u16/i32 to isize, usize to u128, f32 to f64 (lossy) and more.

### Origin

Mirl Extensions simply became too big so a few isolated concepts had to be split. Conversion was one of them.
