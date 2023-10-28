"""Adds the version number to the HTML page. To be used with CI/CD."""

from pathlib import Path


def main():
    pkmn_dir = Path(__file__).parent.parent.parent

    version_path = pkmn_dir / "version.txt"
    version = version_path.read_text().strip()

    index_html_path = pkmn_dir / "net" / "index.html"
    html_content = index_html_path.read_text()

    target_text = "Created by Dudly01"  # NO full-stop!
    html_content = html_content.replace(
        target_text,
        f"{target_text}, version <code>{version}</code>",
    )

    index_html_path.write_text(html_content)
    print(f"Added version '{version}' to the html.")


if __name__ == "__main__":
    main()
