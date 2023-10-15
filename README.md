# pkmn

Data processing tools for Pokémon RBY and GSC.

# Getting started

This section provides a quickstart of using the project.
For more details, visit the documentation of the individual tools.

## Prepare data for `core`

The `core` relies on data taken from Bulbapedia and Smogon.
However, only the data that needed manual preparation is committed to the repo.
The remainder needs to be downloaded via scripts located at `core/scripts` directory:

```
python core/scripts/scrape_smogon.py
python core/scripts/scrape_bulba_images.py
python core/scripts/scrape_bulba_learnsets.py
python core/scripts/evo_chains.py
```

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

## Debug visualization

The CodeLLDB VSCode extension enables running Python scripts
during a debugging session from the Debug Console.
Therefore it is possible to visualize images.

For helpful scripts and more info, peek into `core/scripts/debug_vis.py`.

[CodeLLDB bundles its own copy of Python.](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#installing-packages)
In order to install packages for use in CodeLLDB, use the 
LLDB: Command Prompt command in VSCode, followed by `pip install --user <package>`.


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
