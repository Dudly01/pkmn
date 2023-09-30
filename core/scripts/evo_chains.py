"""Cleans the evo_chains.csv into generation specific evo chains text files.

To create the input file:
 - Visit https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_evolution_family.
 - Copy the table into Open Office Calc (a spreadsheet editor).
 - Verify cell structure and layout. (Look at diverging families and Sylveon).
 - Export into CSV file using comma separators and " quote chars.

The exported file contains the evolution chains line by line.
"""

import csv
from pathlib import Path

JOIN_STR = "->"


def get_full_evo_paths(clean_rows: list[list[str]]) -> list[list[str]]:
    """Returns the full evolution paths from the clean CSV rows.

    Pikachu family full evo path example:
    - [Pikachu family, Pichu, Friendship, Pikachu, Thunder Stone, Raichu (Kantonian)]
    - [Pikachu family, Pichu, Friendship, Pikachu, Thunder Stone (Alola), Raichu (Alolan)]
    """
    paths = []
    path = None
    for row in clean_rows:
        if len(row) != 6:
            raise RuntimeError(
                f"Expected 6 columns, got {len(row)}.",
                "Make sure to pass the cleaned CSV rows.",
            )

        # Check if its a new family
        if row[0]:
            path = []
            path.append(row[0])
            continue  # No more data in family row

        # Find first non-empty cell
        indent = next((i for i, v in enumerate(row) if v), None)
        if indent is None:
            raise ValueError("Found empty row.")

        # Update path with relative path from the current row
        path = path[:indent] + row[indent:]

        paths.append(path)
    return paths


def get_smogon_pokemon(gen: str) -> set[str]:
    """Returns the Pokemon from the Smogon CSV file for the specific generation."""
    gen_data: dict[str, tuple[str, int]] = {
        "i": ("rb", 151),
        "ii": ("gs", 251),
    }

    if gen not in gen_data:
        raise ValueError(f"Gen '{gen}' is not recognised generation.")

    gen_alias, pokemon_count = gen_data[gen]

    script_dir = Path(__file__).parent
    pokedex_path = Path(script_dir.parent, "data", f"smogon_{gen_alias}_pokemon.csv")
    if not pokedex_path.is_file():
        raise RuntimeError(f"Did not find Smogon Pokedex at {pokedex_path}")

    target_header = "name"
    target_col = 0

    pokedex = set()
    with pokedex_path.open("r") as f:
        csv_reader = csv.reader(f, delimiter=",", quotechar='"')

        header = next(csv_reader)
        if (n := header[target_col]) != target_header:
            raise RuntimeError(
                "Unexpected column header at position 0.",
                f"Expected '{target_header}', found '{n}'.",
            )

        for pokemon in (row[target_col] for row in csv_reader):
            pokedex.add(pokemon)

    if (n := len(pokedex)) != pokemon_count:
        raise RuntimeError(
            f"Expected {pokemon_count} pokemon for gen {gen_alias}, found {n}"
        )

    return pokedex


def filter_pokemon(
    full_evo_paths: list[list[str]], pokedex: set[str]
) -> list[list[str]]:
    """Filters the evos to only include the desired pokemon.

    It does not guarantee that every pokemon will have an evolution.
    """
    evo_chain_strings: list[str] = []
    for row in full_evo_paths:
        # The range of columns to keep, [start, stop)
        valid_path_range = None

        for col_idx in range(1, len(row), 2):  # Only look at the PKMN
            pkmn = row[col_idx]

            if pkmn in pokedex:
                if valid_path_range is None:
                    valid_path_range = (col_idx, col_idx + 1)
                valid_path_range = (valid_path_range[0], col_idx + 1)

        if valid_path_range is None:
            continue  # This row is not needed.

        start, stop = valid_path_range

        # Make a string to check for duplicated (sub)chains.
        # Add pre- and postfix to avoid Mew/Mewtwo filtering.
        chain = JOIN_STR + JOIN_STR.join(row[start:stop]) + JOIN_STR

        # O(n+m) solution to make sure no (sub)chain duplications
        # Going in reverse should result in faster hits.
        if not any((chain in c for c in reversed(evo_chain_strings))):
            evo_chain_strings.append(chain)

    evo_chains = [chain.split(JOIN_STR)[1:-1] for chain in evo_chain_strings]

    return evo_chains


def get_evolution_dict(full_evo_paths: list[list[str]]) -> dict:
    """Returns the full evolution paths as nested dicts."""
    result = {}
    for evo in full_evo_paths:
        family = evo[0]
        if family not in result:
            result[family] = {}

        base_pkmn = evo[1]

        if base_pkmn not in result[family]:
            result[family][base_pkmn] = {}

        marcher = result[family][base_pkmn]
        for trigger, evo_pkmn in zip(evo[2:-1:2], evo[3::2]):
            if trigger not in marcher:
                marcher[trigger] = {}
            if evo_pkmn not in marcher[trigger]:
                marcher[trigger][evo_pkmn] = {}
            marcher = marcher[trigger][evo_pkmn]
    return result


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


def merge_unown_lines(csv_rows: list[list[str]]) -> list[list[str]]:
    """Merges the Unown lines, if present, to the expected form.

    The Unown family lists the alphabet forms in the three columns.
    This is unique to this Pokemon, and needs to be cleaned up.
    Works with rows containing either 6 or 8 columns.
    Works with raw and clean rows.
    """

    # Find range of lines for Unown
    begin = next((i for i, r in enumerate(csv_rows) if r[0].strip() == "Unown"), None)
    if begin is None:
        return csv_rows

    end = next(
        (
            i
            for i, r in enumerate(csv_rows[begin + 1 :], start=begin + 1)
            if r[0].strip() != ""
        ),
        len(csv_rows),
    )

    # Create expected Unown lines
    column_count = len(csv_rows[begin])
    new_rows = [
        ["" for _ in range(column_count)],
        ["" for _ in range(column_count)],
    ]
    new_rows[0][0] = "Unown family"
    new_rows[1][1] = "Unown"

    merged_rows = csv_rows[:begin] + new_rows + csv_rows[end:]
    return merged_rows


def get_clean_row(row: list[str]) -> list[str]:
    """Cleans the raw evolutionary CSV rows.

    Including:
    - removing always empty columns
    - renaming Pokemon from Bulbapedia to Smogon
    - removing unwanted characters
    """

    # Remove always empty columns
    row = [c for c in row[0:3] + row[4:6] + row[7:]]

    # Update pokemon names
    row = [bulba_to_smogon_name(c) for c in row]

    # Remove unwanted characters
    row = [c.replace("→", "").replace("*", "").strip() for c in row]

    return row


def main():
    core_dir = Path(__file__).parent.parent
    csv_path = core_dir / "data" / "evo_chains.csv"
    with csv_path.open("r") as f:
        csv_reader = csv.reader(f, delimiter=",", quotechar='"')
        rows = list(csv_reader)

    rows = merge_unown_lines(rows)

    rows = [get_clean_row(r) for r in rows]

    for row in rows:
        for cell in row:
            if JOIN_STR in cell:
                raise RuntimeError(
                    f"JOIN_STR '{JOIN_STR}' found in cell '{cell}'",
                )

    evo_paths = get_full_evo_paths(rows)

    gens = ["i", "ii"]

    for gen in gens:
        pokedex = get_smogon_pokemon(gen)

        filtered_paths = filter_pokemon(evo_paths, pokedex)

        unique_pokemon = set()
        for path in filtered_paths:
            for pokemon in path[::2]:
                unique_pokemon.add(pokemon)

        if len(unique_pokemon) != len(pokedex):
            raise RuntimeError(
                "Mismatching number of Pokemon.",
                f"Evolutions contain {len(unique_pokemon)} pokemon. ",
                f"Pokedex has {len(pokedex)} pokemon.\n",
                f"evos - pokedex: {unique_pokemon - pokedex}\n",
                f"pokedex - evos: {pokedex - unique_pokemon}",
            )

        filtered_paths = [JOIN_STR.join(p) for p in filtered_paths]

        dst_dir = core_dir / "data" / f"gen{gen}_evo_chains.txt"
        with dst_dir.open("w", encoding="utf-8") as f:
            for evo in filtered_paths:
                f.write(evo + "\n")
        print(f"Wrote Gen {gen} evo chains to {dst_dir}")


if __name__ == "__main__":
    main()
