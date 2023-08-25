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


def main():
    script_dir = Path(__file__).parent

    json_path = Path(script_dir, "../data_manual/smogon_rb.json").absolute()
    print(f"Loading JSON from {json_path}")
    with json_path.open("r") as json_file:
        json_content = json_file.read()
    json_content = json.loads(json_content)

    csv_path = Path(script_dir, "../data/moves.csv")
    print(f"Writing moves to {json_path}")
    export_moves(smogon_json=json_content, dst_path=csv_path)

    return


if __name__ == "__main__":
    main()
