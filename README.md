# WasMyRoute

WasMyRoute is a web application that allows users to create, navigate and record their favorite trail routes and cycling routes. The application is designed to operate on web browsers and once it is loaded, can still be used to track and navigate without internet connection. The application can import tracks and export routes in GPX format.

In future, users can create an account and login to view their saved routes. Users can also view other users routes and save them to their account.

## Installation

The project is a WebAssembly applicaton written in Rust. Install `wasmyroute` by:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo new --lib wasmyroute
```

## Deployment

Install `trunk` and other required dependencies

```bash
cargo install --locked trunk
cargo install wasm-bindgen-cli
cargo add web-sys
cargo add log
cargo add seed
```

To deploy this project

```bash
trunk serve
```

## Acknowledgements

TBD

## License

[MIT](https://choosealicense.com/licenses/mit/)

## Running Tests

To run tests, run the following command

```bash
  cargo test
```

## Tech Stack

**Client:** WebAssembly, Rust
