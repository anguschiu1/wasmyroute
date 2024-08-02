[![Build status](https://github.com/anguschiu1/wasmyroute/actions/workflows/ci.yml/badge.svg)]()

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
```

To deploy this project

```bash
trunk serve
```

## Running Tests

To run tests, run the following command:

```bash
cargo test
```

To run wasm-bindgen test, install `cargo install wasm-bindgen-cli` and then run the following command:

```bash
wasm-pack test --safari
```

You will also need to open Safari window to see the result.

Or you can test under headless mode:

```bash
wasm-pack test --headless --firefox
```

## Tech Stack

**Client:** WebAssembly, Rust, Yew

## Acknowledgements

- I learnt so much from him through his [blog](https://blogg.bekk.no/building-an-openstreetmap-app-in-rust-part-i-2adf72c75229)

## Key resources

[Osmium Tool Manual - osmcode](https://osmcode.org/osmium-tool/manual.html)

Very useful OSM export resource - [HOT Export Tool](https://export.hotosm.org/en/v3/exports/new/describe)

Export by countries cities

- [Geofabrik Download Server](http://download.geofabrik.de/europe/great-britain.html)
- [OpenStreetMap](https://www.openstreetmap.org/)

Sketch GPX file - [gpx.studio — the online GPX file editor](https://gpx.studio/)

Script-like tools to fetch OSM - [overpass turbo](https://overpass-turbo.eu/#)

[Rust for beginners: GPX Analyzer - how to find the best GPS tracks in apps like Strava or Nike Run Club](https://nixsanctuary.com/rust-for-beginners-gpx-analyzer-how-to-find-the-best-gps-tracks-in-apps-like-strava-or-nike-run-club/)

## Managing OSM files

### Brew install `osmium` tool to manage OSM files

```bash
brew install osmium-tool
```

Examples:

```bash
osmium fileinfo Cambridgeshire_UK.osm.pbf
osmium fileinfo -e Cambridgeshire_UK.osm.pbf
osmium show Cambridgeshire_UK.osm.pbf
osmium cat Cambridgeshire_UK.osm.pbf -o Cambridgeshire_UK.osm
osmium cat ukmap.osm -o ukmap.osm.pbf
```

Concatenate to stdout for other programs

```bash
osmium cat Cambridgeshire_UK.osm.pbf -f osm | cargo +nightly run
```

where `-f` specified the output format

## License

[MIT](https://choosealicense.com/licenses/mit/)
