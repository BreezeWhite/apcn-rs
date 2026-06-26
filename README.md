# apcn — Arbitrary Precision Constant Numbers

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
![Crates.io Version](https://img.shields.io/crates/v/apcn?style=flat&color=red&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fapcn)
[![WebAssembly Support](https://img.shields.io/badge/WebAssembly-supported-blueviolet.svg)](https://webassembly.org/)

An extremely fast, multi-threaded arbitrary-precision mathematical constant calculator and library written in Rust, with full support for compilation to WebAssembly for JavaScript/TypeScript environments. Calculate mathematical constants like $\pi$ to **1 million decimal digits in less than 200 milliseconds**.

🔗 **Official Web Portal, WASM Playground & Benchmarks**: [https://breezewhite.github.io/apcn-rs/](https://breezewhite.github.io/apcn-rs/)

---

## Features

- **Blazing Fast**: Uses highly optimized algorithms combined with state-of-the-art arbitrary-precision libraries.
- **Parallel Computation**: Leveraging Rayon to perform parallel binary splitting for extreme precision targets (>100k digits).
- **Flexible Backends**:
  - **`rug` (Default)**: Wraps GMP/MPFR/MPC C libraries for maximum CPU performance.
  - **`dashu`**: Pure Rust backend for complete portability, zero system library dependencies, and seamless compilation to WebAssembly.
- **Dual Interface**: Use as a CLI tool or as a library dependency in your Rust projects.

---

## Supported Constants

| Constant | Symbol | CLI Subcommand | Algorithm | Parallelized |
| :--- | :---: | :--- | :--- | :---: |
| **Pi** | $\pi$ | `pi` | Chudnovsky Algorithm (via Binary Splitting) | Yes |
| **Euler's Number** | $e$ | `e` | Taylor Series Expansion (via Binary Splitting) | Yes |
| **Natural Log 2** | $\ln 2$ | `ln2` | Logarithmic Series (via Binary Splitting) | Yes |
| **Natural Log 3** | $\ln 3$ | `ln3` | AGM / Logarithmic Series (via Binary Splitting) | Yes |
| **Natural Log 5** | $\ln 5$ | `ln5` | AGM / Logarithmic Series (via Binary Splitting) | Yes |
| **Square Root 2** | $\sqrt{2}$ | `sqrt2` | Newton-Raphson Method | No |
| **Square Root 3** | $\sqrt{3}$ | `sqrt3` | Newton-Raphson Method | No |
| **Square Root 5** | $\sqrt{5}$ | `sqrt5` | Newton-Raphson Method | No |
| **Golden Ratio Phi** | $\phi$ | `phi` | Direct Calculation | Yes |

---

## CLI Installation & Quick Start

### Installation

#### Pre-built Binary

For Linux / MacOS / Windows (GitBash, MSYS2, Cygwin)
```bash
# Default using GMP(rug) backend
curl -sS https://raw.githubusercontent.com/BreezeWhite/apcn-rs/refs/heads/main/install.sh | sh

# To download dashu version
BACKEND=dashu curl -sS https://raw.githubusercontent.com/BreezeWhite/apcn-rs/refs/heads/main/install.sh | sh
```

For Windows (Powershell)
```shell
irm https://raw.githubusercontent.com/BreezeWhite/apcn-rs/main/install.ps1 | iex

# To download dashu version
$env:BACKEND="dashu"; irm https://raw.githubusercontent.com/BreezeWhite/apcn-rs/main/install.ps1 | iex
```

#### Install from source

Install the CLI binary from source via Cargo.
May need to install additional dependencies and take time to build.

```bash
# GMP backend (default, fast)
# Requires GMP, MPFR, and MPC libraries installed on your system
cargo install apcn

# Pure Rust backed (dashu, portable)
cargo install apcn --no-default-features --features cli,dashu
```

### CLI Command Examples

```bash
# Calculate Pi to the default 1,000 digits
apcn pi

# Calculate Pi to 1,000,000 digits
apcn pi --digits 1000000 

# Calculate Euler's number (e) to 2,000,000 digits in parallel
apcn e --digits 2000000 --parallel

# Calculate Log 2 and benchmark the computation time
apcn ln2 --digits 500000 --bench

# Test the pure speed of the algorithm without string formatting overhead.
apcn sqrt2 -b --no-print

# Get the backend of the built binary
apcn --backend
```

---

## Library Usage

To use `apcn` as a library in your own Cargo project, add it to your `Cargo.toml`:

```toml
[dependencies]
# Use rug (default) for maximum performance
apcn = { version = "0.2" }
```

Or for a pure-Rust, zero-dependency setup:

```toml
[dependencies]
apcn = { version = "0.2", default-features = false, features = ["dashu"] }
```

### Rust Example

```rust
use apcn::{pi, e};

fn main() {
    let digits = 10_000;
    
    // Compute Pi to 10k digits using standard binary splitting
    let pi_val = pi::compute(digits);
    
    // Compute e to 10k digits using parallel binary splitting
    let e_val = e::compute_parallel(digits);
    
    // Convert to a fixed decimal point representation string
    let pi_str = pi_val.to_fixed_string();
    let e_str = e_val.to_fixed_string();
    
    println!("Pi: {}", &pi_str[..50]);
    println!("e:  {}", &e_str[..50]);
}
```

## WebAssembly (WASM) Support

`apcn` is fully compatible with WebAssembly and can be compiled using `wasm-pack` to run directly in web browsers or Node.js.

### NPM Package
The WASM version of `apcn` is compiled with the portable, pure-Rust `dashu` backend (ensuring zero external C dependencies) and is published to the npm registry:

```bash
npm install @breezewhite_yo/apcn
```

For detailed JS/TS integration guides and API examples, refer to the dedicated [WASM README](README_WASM.md).

### Compiling WASM Locally
If you want to build the WebAssembly package from source (requires `wasm-pack` installed):

```bash
make build-wasm
```

This compiles the package to WebAssembly, configures the output under the `@breezewhite_yo` scope, and places the final bundle inside the `pkg/` directory.

---

## Development

### Setup & Testing
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/apc-rs.git
   cd apc-rs
   ```
2. Run standard test suites:
   ```bash
   cargo test
   ```
3. Run test suites with the dashu backend:
   ```bash
   cargo test --no-default-features --features dashu
   ```

### Benchmarks
To compare the performance of `apcn` algorithms against the default implementations built into `rug` or `dashu`:
```bash
# Run benchmarks under default (rug) configuration
cargo bench

# Run benchmarks under dashu configuration
cargo bench --no-default-features --features dashu
```

To build the benchmark page, first run the full bench with above command, then:
```bash
python gh_page/generate_report.py
python -m http.server -d gh_page
```
Open the browser and go to [http://localhost:8000/playground.html](http://localhost:8000/playground.html)

### Feature Flags

| Feature | Description |
| :--- | :--- |
| `cli` | Enables the command-line interface executable. |
| `rug` | Enables the high-performance GMP wrapper backend (mutually exclusive with `dashu`). |
| `dashu` | Enables the portable pure-Rust backend (mutually exclusive with `rug`). |
| `wasm` | Enables WebAssembly bindings support (compiled with wasm-bindgen). |
