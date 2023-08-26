"""
Scrapes the source of the Smogon website.

Steps to create the source JSON:
 - Open the page https://www.smogon.com/dex/rb/pokemon/
 - Look at the Page source
 - Find the <script type="text/javascript"> tag
 - Put the contents (without the variable assignment) into a JSON file
"""

import json
import csv
from pathlib import Path


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
    """Exports the Pokemon data to a CSV file.

    May be enough to export Spc., instead of Spc. Att. abd Spc. Def..
    """

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


def main():
    script_dir = Path(__file__).parent

    json_path = Path(script_dir, "../data_manual/smogon_rb.json").absolute()
    print(f"Loading JSON from {json_path}")
    with json_path.open("r") as json_file:
        json_content = json_file.read()
    json_content = json.loads(json_content)

    csv_path = Path(script_dir, "../data/smogon_rb_moves.csv")
    print(f"Writing moves to {csv_path}")
    export_moves(smogon_json=json_content, dst_path=csv_path)

    csv_path = Path(script_dir, "../data/smogon_rb_pokemon.csv")
    print(f"Writing pokemon to {csv_path}")
    export_pokemon(smogon_json=json_content, dst_path=csv_path)

    return


if __name__ == "__main__":
    main()
