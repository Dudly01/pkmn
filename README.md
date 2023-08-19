# pokemon-dv-calculator
A tool for estimating the DVs of a Pokémon.


# Troubleshooting

## Build WASM code

https://rustwasm.github.io/docs/book/game-of-life/setup.html

cargo install wasm-pack

```
wasm-pack build --target web
```

## Local webserver

To view the examples it's best to start a local webserver from the repo's directory. In Python3, this can be done with:

```
python -m http.server
```

Afterwards, one can visit http://localhost:8000/ which will show `index.html` by default. Other `.html` files can be accessed with their relative paths.

[Source](https://emscripten.org/docs/getting_started/FAQ.html#faq-local-webserver)


## scrap - acquiring screenshot

It was missing libraries.
```
sudo apt-get install libx11-dev libxcb-shm0-dev libxcb-randr0-dev
```

## show-image - showing images

It was missing a packages.
```
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
