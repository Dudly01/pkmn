"""Scrapes the Pokemon and Moves data from Smogon."""

import csv
import json
from pathlib import Path

import requests


def get_smogon_json_string(url: str) -> str:
    """Returns the JSON-like data from the source of the Smogon webpage."""
    response = requests.get(url)
    if response.status_code != 200:
        raise Exception(
            "Failed to fetch the webpage.", f"Status code: {response.status_code}"
        )
    html_text = response.text

    lines = html_text.splitlines()
    json_text = None
    substring_start = "dexSettings = "
    substring_end = "</script>"
    for line in lines:
        if substring_start not in line:
            continue

        start = line.find(substring_start)
        if start == -1:
            raise RuntimeError("Data json start not found in HTML source")
        start += len(substring_start)

        end = line.find(substring_end, start)
        if end == -1:
            # No </script>, then use whole line
            end = len(line)

        json_text = line[start:end]
        break

    if json_text is None:
        raise RuntimeError("Did not find JSON-like data in SMOGON html.")

    return json_text


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
    for move_dict in smogon_json["injectRpcs"][1][1]["moves"]:
        row = [move_dict[col] for col in header]
        rows.append(row)

    rows.sort(key=lambda x: x[0])  # Sort moves by name

    with dst_path.open("w", encoding="utf-8") as f:
        csv_writer = csv.writer(f)

        csv_writer.writerow(header)
        for row in rows:
            csv_writer.writerow(row)


def export_pokemon(smogon_json: dict, dst_path: Path) -> None:
    """Exports the Pokemon data to a CSV file."""

    header = [
        "name",
        "dex_number",
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
    for pkmn_dict in smogon_json["injectRpcs"][1][1]["pokemon"]:
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

    rows.sort(key=lambda x: x[1])  # Sort by dex number

    with dst_path.open("w", encoding="utf-8") as f:
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
    for item_dict in smogon_json["injectRpcs"][1][1]["items"]:
        row = [item_dict[col] for col in header]
        rows.append(row)

    rows.sort(key=lambda x: x[0])  # Sort by name

    with dst_path.open("w", encoding="utf-8") as f:
        csv_writer = csv.writer(f)

        csv_writer.writerow(header)
        for row in rows:
            csv_writer.writerow(row)


def main():
    gen_urls: list[tuple[str, str]] = [
        ("rb", "https://www.smogon.com/dex/rb/pokemon/"),
        ("gs", "https://www.smogon.com/dex/gs/pokemon/"),
    ]

    for gen_name, gen_url in gen_urls:
        smogon_json = get_smogon_json_string(gen_url)
        json_content = json.loads(smogon_json)

        script_dir = Path(__file__).parent
        dst_dir = Path(script_dir.parent, "data")
        if not dst_dir.is_dir():
            print(f"Creating dir at {dst_dir}")
            dst_dir.mkdir()

        csv_path = Path(dst_dir, f"smogon_{gen_name}_moves.csv")
        print(f"Writing moves to {csv_path}")
        export_moves(smogon_json=json_content, dst_path=csv_path)

        csv_path = Path(dst_dir, f"smogon_{gen_name}_pokemon.csv")
        print(f"Writing pokemon to {csv_path}")
        export_pokemon(smogon_json=json_content, dst_path=csv_path)

        csv_path = Path(dst_dir, f"smogon_{gen_name}_items.csv")
        print(f"Writing items to {csv_path}")
        export_items(smogon_json=json_content, dst_path=csv_path)

    return


if __name__ == "__main__":
    main()
