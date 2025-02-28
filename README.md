<h1 align="center">BF Playground</h1>
<div align="center">
 <strong>
   A interpreter and Rust libary for brainf**k
 </strong>
</div>

<br />

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/ZiomekMinecraft/BF-Playground/actions/workflows/rust.yml?query=branch%3Amain">
    <img src="https://img.shields.io/github/actions/workflow/status/ZiomekMinecraft/BF-Playground/rust.yml?branch=main&style=flat-square" alt="actions status" /></a>
  <!-- Version -->
  <a href="https://crates.io/crates/bf_playground">
    <img src="https://img.shields.io/crates/v/bf_playground?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Docs -->
  <a href="https://docs.rs/bf_playground">
  <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/bf_playground">
    <img src="https://img.shields.io/crates/d/bf_playground.svg?style=flat-square" alt="Download" />
  </a>
</div>

## Features

- Interpret Brainf**k code
- Use Brainf**k to print better ;)
- Interpret BF files

## Getting Started

### Prerequisites

- Rust (latest stable version)

### Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/bf_playground.git
cd bf_playground
```

Build the project:

```bash
cargo build
```

### Usage

To use the library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
bf_playground = { path = "path/to/your/bf_playground" }
```

Then, you can use it in your code as follows:

```rust
extern crate bf_playground;

use std::io::{stdin, stdout};
use bf_playground::BFPlayground;

fn main() {
    let code = "++[>++<-]";
    let mut playground = BFPlayground::new();
    playground.execute(code, stdin(), stdout());
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
