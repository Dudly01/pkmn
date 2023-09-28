"""The script downloads Bulbapedia images used in the project.

The articles the images are taken from:
https://bulbapedia.bulbagarden.net/wiki/Summary
https://bulbapedia.bulbagarden.net/wiki/Text_entry_in_the_Pok%C3%A9mon_games
"""

from pathlib import Path
from urllib.parse import urlparse

import requests


def main():
    img_urls = [
        # Summary screens
        "https://archives.bulbagarden.net/media/upload/8/81/Yellow_summary_1.png",
        "https://archives.bulbagarden.net/media/upload/5/57/Yellow_summary_2.png",
        "https://archives.bulbagarden.net/media/upload/0/0d/Crystal_summary_1.png",
        "https://archives.bulbagarden.net/media/upload/f/ff/Crystal_summary_2.png",
        "https://archives.bulbagarden.net/media/upload/d/d7/Crystal_summary_3.png",
        # Latin letters for OCR
        "https://archives.bulbagarden.net/media/upload/f/f0/Nicknaming_I.png",
        "https://archives.bulbagarden.net/media/upload/c/c9/Nicknaming_II.png"
    ]

    script_dir = Path(__file__).parent

    dst_dir = Path(script_dir.parent, "data")
    if not dst_dir.is_dir():
        dst_dir.mkdir()

    for url in img_urls:
        img_data = requests.get(url).content

        filename = urlparse(url).path.rpartition("/")[-1]
        dst_path = dst_dir / filename

        with dst_path.open("wb") as f:
            f.write(img_data)

        print(f"Saved image to {dst_path}")

    print("Done")


if __name__ == "__main__":
    main()
