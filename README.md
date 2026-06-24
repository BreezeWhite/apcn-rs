# apcn — Arbitrary Precision Constant Numbers

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

An extremely fast, multi-threaded arbitrary-precision mathematical constant calculator and library written in Rust. Calculate mathematical constants like $\pi$ to **1 million decimal digits in less than 200 milliseconds**.

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

---

## CLI Installation & Quick Start

### Installation

Install the CLI binary directly from source or via Cargo.

#### Option A: GMP Backend (Default, Fast)
Requires GMP, MPFR, and MPC libraries installed on your system:
```bash
cargo install apcn
```

#### Option B: Pure Rust Backend (Dashu, Portable)
Requires no external system packages:
```bash
cargo install apcn --no-default-features --features cli,dashu
```

### CLI Command Examples

```bash
# Calculate Pi to the default 1,000 digits
apcn pi

# Calculate Pi to 1,000,000 digits
apcn --digits 1000000 pi

# Calculate Euler's number (e) to 2,000,000 digits in parallel
apcn --digits 2000000 --parallel e

# Calculate Log 2 and benchmark the computation time
apcn --digits 500000 --bench ln2

# Calculate Square Root of 2
apcn --digits 100000 sqrt2
```

---

## Library Usage

To use `apcn` as a library in your own Cargo project, add it to your `Cargo.toml`:

```toml
[dependencies]
# Use rug (default) for maximum performance
apcn = { version = "0.1" }
```

Or for a pure-Rust, zero-dependency setup:

```toml
[dependencies]
apcn = { version = "0.1", default-features = false, features = ["dashu"] }
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

---

## Architecture & Algorithm Overview

### Binary Splitting
For series calculations such as $\pi$ (Chudnovsky) and $e$ (Taylor series), the package implements **Binary Splitting** (`bs_utils`). Rather than performing sequential floating-point operations (which accumulate rounding errors and slow down due to high-precision operations), binary splitting converts the series sum into a rational tree structure of integers.
- Reduces high-precision multiplications to a minimum.
- Highly parallelizable: the merge phase splits the computation tree and runs concurrently using Rayon.

### Newton's Method
For square roots ($\sqrt{x}$), `apcn` uses the **Newton-Raphson** method for inverse square root $y \approx 1/\sqrt{x}$ using the iteration:
$$y_{n+1} = y_n \left(1.5 - \frac{x}{2} y_n^2\right)$$
This iteration has quadratic convergence and works directly on floating-point mantissas, scaling up precision dynamically at each step. Finally, we compute $\sqrt{x} = x \cdot y$.

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

### Feature Flags

| Feature | Description |
| :--- | :--- |
| `cli` | Enables the command-line interface executable. |
| `rug` | Enables the high-performance GMP wrapper backend (mutually exclusive with `dashu`). |
| `dashu` | Enables the portable pure-Rust backend (mutually exclusive with `rug`). |

---

## License

This project is licensed under the MIT License.