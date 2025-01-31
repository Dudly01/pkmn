"""Copies the images from net/scripts/img to net/static/img.

The images are present in the former to simplify writing content with markdown.
The images are needed in the latter for publishing the page.
Until I am fine with the content, I do not want to commit the images.
(Yet I am fine committing the index.html..)
"""

import shutil
from pathlib import Path


def main():
    crate_root = Path(__file__).parents[1]
    src_dir = crate_root / "scripts" / "img"
    dst_dir = crate_root / "static" / "img"

    for src in src_dir.iterdir():
        if src.is_dir():
            raise ValueError(f"Did not expect directories in {src_dir}")
        shutil.copy2(src, dst_dir)
    return


if __name__ == "__main__":
    main()
