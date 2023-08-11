"""Exports the evo chains to a txt file from the evo families CSV file.

The CSV file is created from copy-pasting the corresponding Bulbapedia page
into a spreadsheet editor and saving as CSV.
https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_evolution_family

The exported file contains the evolution chains line by line.
E.g. Eevee take up three lines in Gen 1.
"""

import csv
from pathlib import Path


def get_clean_row(row: list[str]) -> list[str]:
    """Returns the cleaned evolutionary CSV row.

    Removes the always empty cells (where the images would be).
    Removes unwanted characters, including leading and trailing whitespaces.
    """
    # Remove always empty columns
    clean_row = [r for r in row[0:3] + row[4:6] + row[7:]]

    # Remove unwanted characters
    clean_row = [r.replace("→", "").replace("*", "").strip() for r in clean_row]

    return clean_row


def get_csv_rows(csv_path: Path):
    """Returns the rows of the evolution families CSV file as-is."""
    column_count = 8
    csv_rows: list[list[str]] = []
    with csv_path.open("r") as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=",", quotechar='"')
        for row in csv_reader:
            if len(row) != column_count:
                raise RuntimeError(f"Expected {column_count} columns, got {len(row)}")
            csv_rows.append(row)
    return csv_rows


def get_full_evolution_paths(csv_rows: list[list[str]]) -> list[list[str]]:
    """Returns the full evolution paths from the CSV rows.

    Pikachu family full evo path example:
    - [Pikachu family, Pichu, Friendship, Pikachu, Thunder Stone, Raichu (Kantonian)]
    - [Pikachu family, Pichu, Friendship, Pikachu, Thunder Stone (Alola), Raichu (Alolan)]
    """
    paths = []
    path = None
    for row in csv_rows:
        row = get_clean_row(row)

        # Check if its a new family
        if row[0]:
            path = []
            path.append(row[0])
            continue  # No more data in family row

        # Find first non-empty cell
        indent = next((i for i, v in enumerate(row) if v), None)
        if indent is None:
            raise ValueError("Found empty row.")

        # Reuse path up until current cell.
        # Because its the same as for the previous CSV row.
        path = path[:indent]

        # Gather non-empty cells, because rows are only equal long due to CSV format
        elements = [cell for cell in row if cell]

        for cell in elements:
            path.append(cell)

        paths.append(path)
    return paths


def get_gen_1_evo_chains(full_evo_paths: list[list[str]]):
    base_stats_path = Path("pokemon_dv_calculator/data/base_stats.csv")
    with base_stats_path.open("r") as f:
        csv_reader = csv.reader(f, delimiter=",", quotechar='"')
        next(csv_reader)  # Skipping header
        gen_1_pkmn = [row[1] for row in csv_reader]
    gen_1_pkmn = set(gen_1_pkmn)

    filtered_evo_chains: list[str] = []
    for row in full_evo_paths:
        # The range of columns to keep, [start, stop)
        valid_path_range = None

        for col_idx in range(1, len(row), 2):  # Only look at the PKMN
            pkmn = row[col_idx]
            pkmn.removesuffix(" (Kantonian)")

            if pkmn in gen_1_pkmn:
                if valid_path_range is None:
                    valid_path_range = (col_idx, col_idx + 1)
                valid_path_range = (valid_path_range[0], col_idx + 1)

        if valid_path_range is None:
            continue  # This row is not needed.

        start, stop = valid_path_range
        chain = ">".join(row[start:stop])

        # O(n+m) solution to make sure no (sub)chain duplications
        # Going in reverse should result in faster hits.
        if not any((chain in c for c in reversed(filtered_evo_chains))):
            filtered_evo_chains.append(chain)

    filtered_evo_chains = [chain.split(">") for chain in filtered_evo_chains]

    return filtered_evo_chains


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
    csv_path = Path("pokemon_dv_calculator/data/evo_families.csv")
    csv_rows = get_csv_rows(csv_path)

    # for row in csv_rows:
    #     print(row)

    evo_paths = get_full_evolution_paths(csv_rows)

    # for path in evo_paths:
    #     print(path)

    evo_paths = get_gen_1_evo_chains(evo_paths)

    # for path in evo_paths:
    #     print(path)

    evo_paths = [">".join(p) for p in evo_paths]

    for path in evo_paths:
        print(path)

    evo_chain_path = Path("evo_chain.txt")
    with evo_chain_path.open("w", encoding="utf-8") as f:
        for evo in evo_paths:
            f.write(evo + "\n")


if __name__ == "__main__":
    main()
