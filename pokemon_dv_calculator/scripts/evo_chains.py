"""Cleans the evo_chains.csv into evo_chains.txt

To create the input file, visit the Bulbapedia "List of Pokémon by evolution family"
page at https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_evolution_family.
Copy the table into a spreadsheet editor, then export it into a CSV file.
(Make sure that the cells are correct for Sylveon.)

The exported file contains the evolution chains line by line.
E.g. Eevee take up three lines in Gen 1.
"""

import csv
from pathlib import Path


JOIN_STR = "->"


def list_clean_rows(csv_path: Path) -> list[list[str]]:
    """Returns the cleaned rows of the evolution families CSV file in a list.

    The cleaning removes unwanted characters and always empty columns.

    Raises RuntimeError if the column count is not 8.
    """
    column_count = 8
    csv_rows: list[list[str]] = []
    with csv_path.open("r") as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=",", quotechar='"')
        for row in csv_reader:
            if len(row) != column_count:
                raise RuntimeError(f"Expected {column_count} columns, got {len(row)}")

            # Remove always empty columns
            clean_row = [r for r in row[0:3] + row[4:6] + row[7:]]

            # Remove unwanted characters
            clean_row = [r.replace("→", "").replace("*", "").strip() for r in clean_row]

            csv_rows.append(clean_row)

    return csv_rows


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


def get_smogon_rb_pokemon() -> set[str]:
    """Returns the Pokemon available in Gen I from the Smogon CSV.

    Note the difference in names:
        Nidoran♀ -> Nidoran-F
        Nidoran♂ -> Nidoran-M
    The smogon_rb_pokemon.csv needs to exist.
    """
    script_dir = Path(__file__).parent
    pokedex_path = Path(script_dir.parent, "data", "smogon_rb_pokemon.csv")
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

    if (n := len(pokedex)) != 151:
        raise RuntimeError(f"Expected 151 pokemon, found {n}")

    return pokedex


def swap_bulba_to_smogon_names(evo_paths: list[list[str]]) -> list[list[str]]:
    """Swaps the Bulbapedia Pokemon names with the Smogon names.

    "(Kantonian)" suffix is removed.

    Only Gen I names:
        Nidoran♀ -> Nidoran-F
    No region swap:
        Raichu (Alolan) -> Raichu-alola
    """

    name_diffs = (
        (" (Kantonian)", ""),
        ("Nidoran♀", "Nidoran-F"),
        ("Nidoran♂", "Nidoran-M"),
    )

    evo_strings: list[str] = []
    for evo in evo_paths:
        string = JOIN_STR.join(p for p in evo)
        evo_strings.append(string)

    swapped_evo_strings: list[str] = []
    for evo in evo_strings:
        for old, new in name_diffs:
            evo = evo.replace(old, new)
        swapped_evo_strings.append(evo)

    swapped_evo_paths: list[list[str]] = []
    for evo in swapped_evo_strings:
        evo = evo.split(JOIN_STR)
        swapped_evo_paths.append(evo)

    return swapped_evo_paths


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


def main():
    script_dir = Path(__file__).parent
    csv_path = Path(script_dir.parent, "data", "evo_chains.csv")
    clean_csv_rows = list_clean_rows(csv_path)

    evo_paths = get_full_evo_paths(clean_csv_rows)

    pokedex = get_smogon_rb_pokemon()

    renamed_paths = swap_bulba_to_smogon_names(evo_paths)

    filtered_paths = filter_pokemon(renamed_paths, pokedex)

    unique_pokemon = set()
    for path in filtered_paths:
        for pokemon in path[::2]:
            unique_pokemon.add(pokemon)

    if (a := len(unique_pokemon)) != (b := len(pokedex)):
        raise RuntimeError(f"Expected {b} Pokemon to have evos, got {a}")

    filtered_paths = [JOIN_STR.join(p) for p in filtered_paths]

    evo_chain_path = Path(script_dir.parent, "data", "rb_evo_chains.txt")
    with evo_chain_path.open("w", encoding="utf-8") as f:
        for evo in filtered_paths:
            f.write(evo + "\n")
    print(f"Createdfile at  {evo_chain_path}")


if __name__ == "__main__":
    main()
