"""Creates the evolution chain txt files expected by core from evo_chains.csv.

The evo_chains.csv file had to be created by hand:
 - Visit https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_evolution_family.
 - Copy the table into Open Office Calc (a spreadsheet editor).
 - Verify cell structure and layout. (Look at diverging families and Sylveon).
 - Export into CSV file using comma separators and " quote chars.

The exported file contains the evolution chains line by line.
This script cleans the entries and filters pokemon based on generation.
"""

import csv
from collections import deque
from pathlib import Path


def gather_evolutions(raw_bulba_table: list[list[str]]) -> dict[str, dict[str, str]]:
    """Returns the evolutions as a {from: {to:how}} dictionary.

    The Pokemon names are in the Bulbapedia style.
    Only leading and trailing whitespaces are removed.
    Nothing else is cleaned.
    """
    evolutions: dict[str, dict[str, str]] = {}

    prev_stage_1_pokemon = None
    prev_stage_2_pokemon = None
    for row in raw_bulba_table:
        row = [elem.strip() for elem in row]

        if row[0]:
            # Only family rows have data in first column
            continue

        # Stage 1 pokemon
        name = row[1]

        # Handle unique Unown entries
        if name.startswith("Unown"):
            if name not in evolutions:
                evolutions["Unown"] = {}
            continue

        if name:
            if name not in evolutions:
                evolutions[name] = {}
            prev_stage_1_pokemon = name

        # Stage 2 pokemon
        name = row[4]
        if name:
            if name not in evolutions:
                evolutions[name] = {}
            prev_stage_2_pokemon = name
            evo_trigger = row[2]
            evolutions[prev_stage_1_pokemon][name] = evo_trigger

        # Stage 3 pokemon
        name = row[7]
        if name:
            if name not in evolutions:
                evolutions[name] = {}
            evo_trigger = row[5]
            evolutions[prev_stage_2_pokemon][name] = evo_trigger

    return evolutions


def bulba_to_smogon_name(bulba_name: str) -> str:
    """Returns the Smogon name from the Bulbapedia name."""

    to_switch: list[tuple[str, str]] = [
        ("♀", "-F"),
        ("♂", "-M"),
        (" (Kantonian)", ""),
        (" (Johtonian)", ""),
        # (" (Alolan)", "-Alola")
        # (" (Galarian)", "-Galar")
        # (" (Hisuian)", "-Hisui")
        # (" (Paldean, Combat Breed)", "-Paldea-Combat")
        # (" (Paldean, Blaze Breed)", "-Paldea-Blaze")
        # (" (Paldean, Aqua Breed)", "-Paldea-Aqua")
        # (" (Paldean)", "-Paldea")
    ]
    smogon_name = bulba_name
    for a, b in to_switch:
        smogon_name = smogon_name.replace(a, b)
    return smogon_name


def clean_evolution_trigger(trigger_text: str) -> str:
    clean_text = trigger_text.removesuffix(" →")
    return clean_text


def clean_evolutions(
    raw_evolutions: dict[str, dict[str, str]],
    smogon_pokedex: set[str],
) -> dict[str, dict[str, str]]:
    """Switches to Smogon naming, keeps pokemon found in the dex and cleans the evo triggers."""
    filtered_evos: dict[str, dict[str, str]] = {}
    for name, evolves_into in raw_evolutions.items():
        smogon_name = bulba_to_smogon_name(name)
        if smogon_name not in smogon_pokedex:
            continue
        if smogon_name not in filtered_evos:
            filtered_evos[smogon_name] = {}

        curr_poke_evos = filtered_evos[smogon_name]
        for evolution_name, evolution_trigger in evolves_into.items():
            smogon_evo_name = bulba_to_smogon_name(evolution_name)
            if smogon_evo_name not in smogon_pokedex:
                continue
            cleaned_trigger = clean_evolution_trigger(evolution_trigger)

            # Unique cleaning
            if smogon_evo_name == "Quilava":
                # "Level 14/17LA"
                cleaned_trigger = cleaned_trigger.removesuffix("/17LA")
            if smogon_evo_name in ("Espeon", "Umbreon"):
                # "Friendship (day);\nLevel up with Sun ShardXD"
                # "Friendship (night);\nLevel up with Moon ShardXD"
                semi_colon_pos = cleaned_trigger.find(";")
                cleaned_trigger = cleaned_trigger[:semi_colon_pos]

            curr_poke_evos[smogon_evo_name] = cleaned_trigger
    return filtered_evos


def create_evo_chains(evolutions: dict[str, dict[str, str]]) -> list[str]:
    """Returns the list of complete evolution chains."""
    complete_paths: list[str] = []
    pokemons_to_visit = deque(((name, name) for name in evolutions.keys()))
    pokemon_visited: set[str] = set()
    while pokemons_to_visit:
        curr_pokemon, evo_path = pokemons_to_visit.popleft()

        if curr_pokemon in pokemon_visited:
            # Already visited this pokemon
            continue
        pokemon_visited.add(curr_pokemon)

        if len(evolutions[curr_pokemon]) == 0:
            # No more evolutions, path is complete
            complete_paths.append(evo_path)
            continue

        for evo_name, evo_trigger in reversed(evolutions[curr_pokemon].items()):
            # Reverse iter to compensate FILO deque
            curr_evo_path = f"{evo_path}->{evo_trigger}->{evo_name}"
            pokemons_to_visit.appendleft((evo_name, curr_evo_path))

    return complete_paths


def main():
    data_dir = Path(__file__).parents[1] / "data"

    bulba_evo_table_path = data_dir / "evo_chains.csv"
    with bulba_evo_table_path.open("r", encoding="utf-8") as f:
        csv_reader = csv.reader(f, delimiter=",", quotechar='"')
        raw_table = list(csv_reader)

    evolutions = gather_evolutions(raw_table)

    # evo_json = json.dumps(evolutions, indent=4, ensure_ascii=False)
    # (data_dir / "debug.json").write_text(evo_json, encoding="utf-8")

    smogon_gen_1_pokedex_path = data_dir / "smogon_rb_pokemon.csv"
    with smogon_gen_1_pokedex_path.open("r", encoding="utf-8") as f:
        csv_reader = csv.reader(f)
        next(csv_reader)  # skip header
        pokedex_gen_1 = set((row[0] for row in csv_reader))

    smogon_gen_2_pokedex_path = data_dir / "smogon_gs_pokemon.csv"
    with smogon_gen_2_pokedex_path.open("r", encoding="utf-8") as f:
        csv_reader = csv.reader(f)
        next(csv_reader)  # skip header
        pokedex_gen_2 = set((row[0] for row in csv_reader))

    filtered_evos = clean_evolutions(evolutions, pokedex_gen_1)
    # evo_json = json.dumps(filtered_evos, indent=4, ensure_ascii=False)
    # (data_dir / "debug_rb.json").write_text(evo_json, encoding="utf-8")

    evo_chains = create_evo_chains(filtered_evos)
    evo_txt = "\n".join(evo_chains)
    evo_txt_path = data_dir / "geni_evo_chains.txt"
    evo_txt_path.write_text(evo_txt, encoding="utf-8")
    print(f"Created {evo_txt_path}")

    filtered_evos = clean_evolutions(evolutions, pokedex_gen_2)
    # evo_json = json.dumps(filtered_evos, indent=4, ensure_ascii=False)
    # (data_dir / "debug_gs.json").write_text(evo_json, encoding="utf-8")

    evo_chains = create_evo_chains(filtered_evos)
    evo_txt = "\n".join(evo_chains)
    evo_txt_path = data_dir / "genii_evo_chains.txt"
    evo_txt_path.write_text(evo_txt, encoding="utf-8")
    print(f"Created {evo_txt_path}")


if __name__ == "__main__":
    main()
