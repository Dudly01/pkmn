"""
Creates a CSV with the move data taken from SMOGON.

Steps to create the source JSON:
 - Open the page https://www.smogon.com/dex/rb/moves/
 - Look at the source
 - Find the <script type="text/javascript"> tag
 - Put the contents (without the variable assignment) into a JSON file
"""

import json
import csv
from pathlib import Path


def main():
    json_path = Path("pokemon_dv_calculator/data/moves.json")
    with json_path.open("r") as json_file:
        json_content = json_file.read()
        json_content = json.loads(json_content)

    header = [
        "name",
        "type",
        "category",
        "power",
        "accuracy",
        "pp",
        "description",
    ]

    moves = []
    for move_dict in json_content["injectRpcs"][1][1]["moves"]:
        move = [move_dict[col] for col in header]
        moves.append(move)

    moves.sort(key=lambda x: x[0])  # Sort moves by name

    csv_path = Path("moves.csv")
    with csv_path.open("w", encoding="utf-8") as f:
        csv_writer = csv.writer(f)

        csv_writer.writerow(header)
        for move in moves:
            csv_writer.writerow(move)

    return


if __name__ == "__main__":
    main()
