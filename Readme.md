# Zero-Knowledge Proof Sudoku Verifier

This Rust project demonstrates a simple Zero-Knowledge Proof (ZKP) system for verifying Sudoku solutions without revealing the actual solution. It uses a 4x4 Sudoku board for simplicity.

## Features

- Zero-knowledge verification of Sudoku solutions
- Simplified 4x4 Sudoku board
- Demonstrates core ZKP concepts: commitment, challenge, response, and verification

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo (Rust's package manager, included with Rust)

## Dependencies

This project uses the following external crates:

- `rand`: For random number generation
- `sha2`: For SHA-256 hashing

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/lordx64/zkp-sudoku.git
   cd zkp-sudoku
   ```

2. Build the project:
   ```
   cargo build
   ```

## Usage

Run the program with:
