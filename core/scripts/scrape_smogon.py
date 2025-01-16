"""Scrapes the Pokemon, Items and Moves data from Smogon."""

import csv
import json
import re
from pathlib import Path

import requests


def get_smogon_json(url: str) -> dict[str, list]:
    """Returns the JSON-like data from the Smogon webpage."""
    response = requests.get(url)
    if response.status_code != 200:
        raise Exception(
            f"Failed to fetch the webpage. Status code: {response.status_code}"
        )
    html_text = response.text

    # The JSON data is found between
    # <script type="text/javascript">dexSettings =
    # and
    # </script>
    pattern = r'<script\s+type="text/javascript">\s*dexSettings\s*=\s*(.*?)</script>'
    matches = list(re.finditer(pattern, html_text))

    if not matches:
        raise RuntimeError("Could not find JSON data on Smogon website")
    if len(matches) > 1:
        raise RuntimeError(f"Expected 1 JSON data from Smogon, got {len(matches)}")

    json_text = matches[0].group(1)  # Only need content of (.*?)
    json_data = json.loads(json_text)

    try:
        smogon_data = json_data["injectRpcs"][1][1]
    except {KeyError, IndexError, TypeError} as e:
        raise RuntimeError(f"Smogon JSON has unexpected format: {e}")

    # Similar elements to sidebar of webpage
    expected_elems = (
        "pokemon",
        "moves",
        "abilities",
        "items",
        "types",
        "formats",
        "natures",
        "moveflags",
    )
    for elem in expected_elems:
        if elem not in smogon_data:
            raise ValueError(f"Smogon JSON has unexpected format: {elem} not found")

    return smogon_data


def export_moves(smogon_json: dict, dst_path: Path) -> None:
    """Exports the move data to a CSV file."""

    header = [
        "name",
        "type",
        "category",
        "power",
        "accuracy",
        "pp",
        "description",
    ]

    rows = []
    for move_dict in smogon_json["moves"]:
        row = [move_dict[col] for col in header]
        rows.append(row)

    rows.sort(key=lambda x: x[0])  # Sort moves by name

    # Specifying newline avoids extra lines on Windows
    with dst_path.open("w", encoding="utf-8", newline="") as f:
        csv_writer = csv.writer(f)

        csv_writer.writerow(header)
        for row in rows:
            csv_writer.writerow(row)


def export_pokemon(smogon_json: dict, dst_path: Path) -> None:
    """Exports the Pokemon data to a CSV file."""

    header = [
        "name",
        "ndex",
        "type1",
        "type2",
        "hp",
        "atk",
        "def",
        "spa",
        "spd",
        "spe",
    ]

    rows = []
    for pkmn_dict in smogon_json["pokemon"]:
        row = [
            pkmn_dict["name"],
            pkmn_dict["oob"]["dex_number"],
            pkmn_dict["types"][0],
            pkmn_dict["types"][1] if len(pkmn_dict["types"]) == 2 else "",
            pkmn_dict["hp"],
            pkmn_dict["atk"],
            pkmn_dict["def"],
            pkmn_dict["spa"],
            pkmn_dict["spd"],
            pkmn_dict["spe"],
        ]
        rows.append(row)

    if len(rows) not in (151, 251):
        raise ValueError(f"Expected 151 or 251 pokemons, got {len(rows)}")

    rows.sort(key=lambda x: x[1])  # Sort by dex number

    # Specifying newline avoids extra lines on Windows
    with dst_path.open("w", encoding="utf-8", newline="") as f:
        csv_writer = csv.writer(f)

        csv_writer.writerow(header)
        for row in rows:
            csv_writer.writerow(row)


def export_items(smogon_json: dict, dst_path: Path) -> None:
    """Exports the item data to a CSV file."""

    header = [
        "name",
        "description",
    ]

    rows = []
    for item_dict in smogon_json["items"]:
        row = [item_dict[col] for col in header]
        rows.append(row)

    rows.sort(key=lambda x: x[0])  # Sort by name

    # Specifying newline avoids extra lines on Windows
    with dst_path.open("w", encoding="utf-8", newline="") as f:
        csv_writer = csv.writer(f)

        csv_writer.writerow(header)
        for row in rows:
            csv_writer.writerow(row)


def main():
    gen_urls: list[tuple[str, str]] = [
        ("rb", "https://www.smogon.com/dex/rb/pokemon/"),
        ("gs", "https://www.smogon.com/dex/gs/pokemon/"),
    ]

    dst_dir = Path(__file__).parents[1] / "data"
    if not dst_dir.is_dir():
        print(f"Creating dir at {dst_dir}")
        dst_dir.mkdir()

    for gen_name, gen_url in gen_urls:
        print(f"Preparing data for gen {gen_name}")
        smogon_json = get_smogon_json(gen_url)

        csv_path = dst_dir / f"smogon_{gen_name}_pokemon.csv"
        print(f"  Writing pokedex to {csv_path}")
        export_pokemon(smogon_json=smogon_json, dst_path=csv_path)

        csv_path = dst_dir / f"smogon_{gen_name}_moves.csv"
        print(f"  Writing movedex to {csv_path}")
        export_moves(smogon_json=smogon_json, dst_path=csv_path)

        csv_path = dst_dir / f"smogon_{gen_name}_items.csv"
        print(f"  Writing itemdex to {csv_path}")
        export_items(smogon_json=smogon_json, dst_path=csv_path)

    return


if __name__ == "__main__":
    main()
