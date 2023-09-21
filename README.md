# pokemon-dv-calculator

A tool for estimating the DVs of a Pokémon.

# Getting started

This section provides a quickstart of using the project.
For more details, visit the documentation of the individual tools.

## Building WASM, testing website

```
# Installs wasm-pack
cargo install wasm-pack

# Builds WASM code
wasm-pack build --target web

# Starts local webserver
python -m http.server
```

Visit http://localhost:8000/ to see `index.html`.
Access other `.html` files with their relative path.

Hard-refresh Firefox with `Ctrl + F5` in case changes do not show up.

## Benchmarks

The project uses [Criterion.rs](https://github.com/bheisler/criterion.rs).

```
# Runs benchmarks
cargo bench

# Filters benchmark IDs with <filter> regular expression
cargo bench -- <filter>

# Saves baseline
cargo bench -- --save-baseline <name>

# Compares against baseline
cargo bench -- --baseline <name>
```

# Troubleshooting

This section provides information on issues encountered during the develpment.

## Missing dependencies

Some crates may require the installation of certain libraries.

```
# For scrap
sudo apt-get install libx11-dev libxcb-shm0-dev libxcb-randr0-dev

# For show-image
sudo apt-get install pkg-config libfontconfig1-dev
```

## Cargo version conflict

As [comment mentiones](https://github.com/serde-rs/json/issues/409#issuecomment-362696245), update the crates:
```
cargo update
```

# Sources

## Pokemon
Bulbapedia https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_base_stats_(Generation_I)
Neoseeker https://www.neoseeker.com/pokemon-red/faqs/2740069-pokemon-rb-save-state-hacking.html
Smogon https://www.smogon.com/ingame/guides/rby_gsc_stats

## JS app 
https://dev.to/bibekkakati/capture-screen-and-stream-like-zoom-using-javascript-1b65
https://developer.mozilla.org/en-US/docs/Web/API/Screen_Capture_API/Using_Screen_Capture
