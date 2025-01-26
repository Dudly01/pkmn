# data

This project would not have been possible without data from other sources.

The learnsets and evolutions originate from [Bulbapedia](https://bulbapedia.bulbagarden.net). The moves, items and Pokémon originate from [Smogon](https://www.smogon.com/). These files, therefore, follow the licensing rules of their respective website of origin. The list of in-game item names, used in code and not present as an individual file, originates from [Serebii.net](https://serebii.net/).

The screenshots in the `images` folder are direct captures from Gen I and II Pokémon games. They are used under fair use for non-commercial purposes.

The `bulba_evo_chains.csv` file was created manually:

 - Visit the [evolutionary line article on Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_evolution_family).
 - Copy the table into Open Office Calc (a spreadsheet editor). I did not have success with Excel.
 - Verify the layout and the structure of the cells; we need the form that we see on the website. (Look at diverging families and Sylveon).
 - Export sheet into a CSV using comma separators and " quote chars.

The rest of the data files were created by scripts. `core/scripts/scrape_smogon.py` creates the CSV files for the itemdex, the movedex and the pokedex. `core/scripts/scrape_bulba_learnsets.py` creates the learnset files **using the Smogon naming convention** for the Pokemon. `core/scripts/create_evo_chains.py` creates the text files for the generation specific evolution chains from `bulba_evo_chains.csv`, **using the Smogon naming convention** for the Pokémon.
