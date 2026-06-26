# @breezewhite_yo/apcn — Arbitrary Precision Constant Numbers (WASM)

[![npm version](https://img.shields.io/npm/v/@breezewhite_yo/apcn.svg)](https://www.npmjs.com/package/@breezewhite_yo/apcn)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

An extremely fast mathematical constant calculator compiled to WebAssembly (WASM). Compute constants like $\pi$ or $e$ to thousands of decimal digits directly in modern browsers and JavaScript/TypeScript environments with blazing-fast speed.

This package wraps the Rust `apcn` crate, using the pure-Rust `dashu` arbitrary-precision backend for complete compatibility and zero external system dependencies.

---

## Installation

Install the package via npm:

```bash
npm install @breezewhite_yo/apcn
```

---

## Quick Start (Browser ES Modules)

```html
<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <title>apcn WASM Example</title>
</head>
<body>
    <script type="module">
        import init, { compute_pi, ln, sqrt } from "./node_modules/@breezewhite_yo/apcn/apcn.js";

        async function run() {
            // Initialize the WebAssembly module
            await init();

            // Calculate Pi to 10,000 decimal places
            const pi10k = compute_pi(10000);
            console.log("Pi (first 50 digits):", pi10k.slice(0, 52));

            // Calculate natural logarithm of 2 to 100 decimal places
            const ln2 = ln(2.0, 100);
            console.log("ln(2):", ln2);

            // Calculate the square root of 5 to 50 decimal places
            const sqrt5 = sqrt(5, 50);
            console.log("sqrt(5):", sqrt5);
        }

        run().catch(console.error);
    </script>
</body>
</html>
```

---

## API Reference

The WASM module exports the following functions. All precision values `prec` represent the requested number of digits **after** the decimal point.

### `compute_pi(prec: number): string`
Computes the value of $\pi$ (Pi) using the Chudnovsky algorithm.
- **`prec`**: Number of decimal places after the decimal point.

### `compute_e(prec: number): string`
Computes the value of $e$ (Euler's number) using Taylor series expansion.
- **`prec`**: Number of decimal places after the decimal point.

### `compute_sqrt2(prec: number): string`
Computes the value of $\sqrt{2}$ (Square root of 2) using the Newton-Raphson method.
- **`prec`**: Number of decimal places after the decimal point.

### `sqrt(x: number, prec: number): string`
Computes the value of $\sqrt{x}$ (Square root of an integer $x$) using the Newton-Raphson method.
- **`x`**: Input integer.
- **`prec`**: Number of decimal places after the decimal point.

### `ln(x: number, prec: number): string`
Computes the value of $\ln(x)$ (Natural logarithm of a float $x$).
- **`x`**: Input floating-point value.
- **`prec`**: Number of decimal places after the decimal point.

### `compute_ln2(prec: number): string`
Computes the value of $\ln(2)$ (Natural logarithm of 2).
- **`prec`**: Number of decimal places after the decimal point.

### `compute_phi(prec: number): string`
Computes the value of $\phi$ (Golden Ratio) using Fibonacci/Direct formula.
- **`prec`**: Number of decimal places after the decimal point.

---

## License

Licensed under the MIT License.
