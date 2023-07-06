# pokemon-dv-calculator
A tool for estimating the DVs of a Pokémon.


# Troubleshooting

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

Bulbapedia https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_base_stats_(Generation_I)
Pokémon base stats https://www.neoseeker.com/pokemon-red/faqs/2740069-pokemon-rb-save-state-hacking.html
