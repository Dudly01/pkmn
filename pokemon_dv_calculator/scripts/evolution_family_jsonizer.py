"""Creates a JSON from the evolution family CSV file.

The CSV file is created from copy-pasting the corresponding Bulbapedia page
into a spreadsheet editor and saving as CSV.
https://bulbapedia.bulbagarden.net/wiki/List_of_Pok%C3%A9mon_by_evolution_family
"""

import csv
import json
from pathlib import Path

COL_COUNT = 8


def get_clean_row(row: list[str]) -> list[str]:
    """Returns the cleaned CSV row.

    Removes the always empty cells (where the images would be).
    Removes unwanted characters, including leading and trailing whitespaces.
    """
    # Remove always empty columns
    clean_row = [r for r in row[0:3] + row[4:6] + row[7:]]

    # Remove unwanted characters
    clean_row = [r.replace("→", "").replace("*", "").strip() for r in clean_row]

    return clean_row


def get_csv_rows(csv_path: Path):
    """Returns the cleaned rows of the evolution families CSV file."""
    csv_rows: list[list[str]] = []
    with csv_path.open("r") as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=",", quotechar='"')
        for row in csv_reader:
            if len(row) != COL_COUNT:
                raise RuntimeError(f"Expected {COL_COUNT} columns, got {len(row)}")

            row = get_clean_row(row)

            csv_rows.append(row)
    return csv_rows


def get_full_evolution_paths(csv_rows: list[list[str]]) -> list[list[str]]:
    """Returns the full evolution paths from the CSV rows."""
    paths = []
    path = None
    for row in csv_rows:
        # Check if its a new family
        if row[0]:
            path = []
            path.append(row[0])
            continue  # No more data in row

        # See how much of the path needs to be used
        indent = next((i for i, v in enumerate(row) if v), None)
        if indent is None:
            raise ValueError("Found empty row.")

        path = path[:indent]

        # Gather non-empty cells
        elements = [cell for cell in row if cell]

        for cell in elements:
            path.append(cell)

        paths.append(path)
    return paths


def main():
    csv_path = Path("pokemon_dv_calculator/data/evolution_families.csv")
    csv_rows = get_csv_rows(csv_path)

    # for row in csv_rows:
    #     print(row)

    evo_paths = get_full_evolution_paths(csv_rows)

    for path in evo_paths:
        print(path)


if __name__ == "__main__":
    main()
