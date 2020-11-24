# Readme

This will mostly be tech demos for the Rust + WebAssembly + WebGL stack. If anything useful comes of this repo,
we can extract it to a separate package and build from there.

## TL;DR

- You need:
  - [Rust](https://www.rust-lang.org/tools/install)
  - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
  - [npm](https://www.npmjs.com/get-npm)
- Steps:
  1. Build:
      ```bash
      wasm-pack build
      ```
  2. Test:
      ```bash
      wasm-pack test --headless --firefox
      ```
  3. Install frontend dependencies
      ```bash
      # From ./www
      npm install
      ```
  4. Run server:
      ```bash
      # From ./www
      npm run start
      ```

## Before You Start

If you're unfamiliar with WebAssembly and Rust, you may want to get started
with [Conway's Game of Life Tutorial](https://rustwasm.github.io/book/game-of-life/introduction.html).

If you want to get an idea of how Rust, WebGL, and WebAssembly interact, follow this
[Basic Water Tutorial](https://www.chinedufn.com/3d-webgl-basic-water-tutorial/).

[Rust Playground](https://play.rust-lang.org/)

## Basic Structure

- `./src` contains the Rust source code.
- `./tests` contains tests for our Rust code.
- `./www` contains the code for our web server.
