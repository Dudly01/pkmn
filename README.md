# pkmn

pkmn is a set of tools for Generation I (Red, Blue, and Yellow) and Generation II (Gold, Silver, and Crystal) Game Boy Pokémon games.

It aims to simplify finding learnsets, evolutions and the stats of the known moves and to simplify calculating DVs. No need to enter data manually, pkmn reads the info directly from the Game Boy screen. [Try the webapp here.](https://dudly01.github.io/pkmn/)

It aims to simplify looking up information for the Pokémon that is currently inspected on the Game Boy screen. The information includes the evolutions, learnsets, stats of the known moves and the possible DVs. [Try the webapp here.](https://dudly01.github.io/pkmn/)

pkmn 


## Getting started

This section describes the steps of using the project from its source on Linux. User experience on Windows may vary.

### Dependencies

Certain crates may require the installation of additional libraries. These are the ones I had to get for my system:

```sh
# For scrap
sudo apt-get install libx11-dev libxcb-shm0-dev libxcb-randr0-dev

# For show-image
sudo apt-get install pkg-config libfontconfig1-dev
```

### Prepare data

The project uses data from Bulbapedia and Smogon. As not all of it has been commited (yet) to the repo, the preparation scripts need to be called manually.

To do so, first, install the required Python packages. Using conda, it can be done with:

```sh
conda install --yes --file core/scripts/requirements.txt  
```

With pip, this would change to:

```sh
pip install -r core/scripts/requirements.txt
```

Afterwards, the necessary scripts can be run with:

```sh
python core/scripts/scrape_smogon.py;
python core/scripts/scrape_bulba_images.py;
python core/scripts/scrape_bulba_learnsets.py;
python core/scripts/create_evo_chains.py;
```

The data should be ready within 1-2 minutes.

### Build desktop app

The `desktop` example located in the `core` package is a desktop version of the webapp. It locates the Game Boy on the primary display and prints the results to the terminal. To run the app, navigate to the **core directory** and run:

```sh
cargo run --example desktop --release
```

Make sure the terminal is large enough for the text to fit! 

#### Other examples

The other app in `core/examples` were primarily used for development. From the `core` directory, they can be listed with:

```sh
cargo run --example
```

To run one, use:

```sh
cargo run --example <example_name> --release
```

However, there is likely little use of them to others.

### Build WASM and test webapp locally

The projects uses wasm-pack to build the WebAssembly (WASM) package. To install it, use cargo:

```sh
cargo install wasm-pack
```

To build the package, navigate to the **net directory** and run:

```sh
wasm-pack build --target web
```

Afterwards, start a local webserver with:

```sh
python -m http.server
```

The webapp can be accessed by visiting [http://localhost:8000/](http://localhost:8000/) (8000 is the port selected by default).

Using [http://0.0.0.0:8000/](http://0.0.0.0:8000/) may prevent the screen-sharing from working. If changes do not show up, try hard-refreshing the page (Ctrl + F5 in Firefox).

### Benchmarks

The project uses [Criterion.rs](https://github.com/bheisler/criterion.rs).

```sh
# Runs benchmarks
cargo bench

# Filters benchmark IDs with <filter> regular expression
cargo bench -- <filter>

# Saves baseline
cargo bench -- --save-baseline <name>

# Compares against baseline
cargo bench -- --baseline <name>
```

The benchmarks are not exhaustive.

### Debug visualization

**No longer seems to work: lldb.process is None when reading from memory.**

The VS Code extension called CodeLLDB enables users to run Python scripts during a debugging session. With this, it is possible to plot images and to inspect them visually. For more info, visit [core/scripts/debug_vis.py](core/scripts/debug_vis.py).

## Troubleshooting

This section provides information on issues encountered during the development.

### Cargo version conflict

As [comment mentiones](https://github.com/serde-rs/json/issues/409#issuecomment-362696245), update the crates with:

```sh
cargo update
```

## References

This section encloses the useful sources used in the development of the project.

Pokémon data taken from
[Bulbapedia](https://bulbapedia.bulbagarden.net/),
[Serebii](https://www.serebii.net/),
[Smogon](https://www.smogon.com/),
([Neoseeker](https://www.neoseeker.com/pokemon-red/faqs/2740069-pokemon-rb-save-state-hacking.html) in early stages) 
.

Pokémon is a trademark of Nintendo.

Useful webdev resources include 
[MDN Web Docs](https://developer.mozilla.org/),
CSS-TRICKS' 
[Flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/)
and 
[Grid](https://css-tricks.com/snippets/css/complete-guide-grid/)
guides.

Website aesthetics were heavily inspired by the 
[MDN Web Docs](https://developer.mozilla.org/)
.

Honorary mentions:
- https://dev.to/bibekkakati/capture-screen-and-stream-like-zoom-using-javascript-1b65
- https://developer.mozilla.org/en-US/docs/Web/API/Screen_Capture_API/Using_Screen_Capture
