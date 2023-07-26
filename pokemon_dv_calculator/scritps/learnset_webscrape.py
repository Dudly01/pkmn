"""
Collects the Gen 1 learnsets from Bulbapedia into a JSON file.
Currently only the "By leveling up" learnset is collected.
"""

import json
from pathlib import Path
from urllib.parse import urljoin

import bs4
import requests
from bs4 import BeautifulSoup
from tqdm import tqdm


def get_learnset_article_urls() -> list[str]:
    """Returns the URLs of the Gen1 learnset articles for each 151 Pokemon."""

    # The category website
    category_url = "https://bulbapedia.bulbagarden.net/wiki/Category:Pok%C3%A9mon_learnsets_(Generation_I)"

    response = requests.get(category_url)
    if response.status_code != 200:
        raise Exception(
            "Failed to fetch the webpage.", f"Status code: {response.status_code}"
        )

    soup = BeautifulSoup(response.content, "html.parser")

    article_urls: list[str] = []
    for ul in soup.find_all("ul"):  # unordered list
        for li in ul.find_all("li"):  # list item
            for a in li.find_all("a"):
                text: str = a.contents[0]
                if text.endswith(" (Pokémon)/Generation I learnset"):
                    relative_url = a["href"]
                    category_url = urljoin(
                        "https://bulbapedia.bulbagarden.net", relative_url
                    )
                    article_urls.append(category_url)

    return article_urls


def article_url_to_source_url(url: str) -> str:
    """Returns the Bulbapedia source page url from the article page url.

    Example:
    https://bulbapedia.bulbagarden.net/wiki/Squirtle_(Pok%C3%A9mon)/Generation_I_learnset
    https://bulbapedia.bulbagarden.net/w/index.php?title=Squirtle_(Pok%C3%A9mon)/Generation_I_learnset&action=edit

    Wikipedia seems to have the same system:
    https://en.wikipedia.org/wiki/Way_of_the_Lighthouses
    https://en.wikipedia.org/w/index.php?title=Way_of_the_Lighthouses&action=edit
    """

    title = url.removeprefix("https://bulbapedia.bulbagarden.net/wiki/")
    url_source = (
        "https://bulbapedia.bulbagarden.net/w/index.php?title=" + title + "&action=edit"
    )
    return url_source


def get_wiki_article_markdown_source(url: str) -> str:
    """Returns the markdown source of the Bulbapedia article as a string."""

    source_url = article_url_to_source_url(url=url)

    response = requests.get(source_url)
    if response.status_code != 200:
        raise Exception(
            f"Failed to fetch the webpage. Status code: {response.status_code}"
        )

    soup = BeautifulSoup(response.content, "html.parser")

    textarea_tags = soup.find_all("textarea")
    if len(textarea_tags) != 1:
        raise RuntimeError(f"Expected one textarea tag. Found {len(textarea_tags)}")

    textarea: bs4.element.Tag = textarea_tags[0]
    text = textarea.get_text()

    return text


def get_pkmn(markdown_source: str) -> tuple[str, str]:
    """Returns the Pokemon Ndex number and name from the Wiki markdown source."""
    ndex = None
    pkmn = None

    for line in markdown_source.splitlines():
        if line.startswith("{{pokelinkback|"):
            line_elems = line.split("|")
            ndex = line_elems[1]
            pkmn = line_elems[2]
            break

    if ndex is None or pkmn is None:
        raise ValueError("Could not find Pokemon")

    return ndex, pkmn


def get_level_learnset(markdown_source: str) -> list[list[str]]:
    """Extracts the "By leveling up" learnset table from the WIKI markdown source.

    The returned table contains the header and the rows, column by column.
    """

    markdown_rows = markdown_source.splitlines()

    table_headers: list[str] = []
    table_rows: list[str] = []
    table_footer: list[str] = []
    for row in markdown_rows:
        if row.startswith("{{learnlist/levelh"):
            table_headers.append(row)
            continue

        # Macro differs when Y version has different learnset
        if row.startswith("{{learnlist/level1") or row.startswith("{{learnlist/levelI"):
            table_rows.append(row)
            continue

        if row.startswith("{{learnlist/levelf"):
            table_footer.append(row)
            continue

    if len(table_headers) != 1:
        raise RuntimeError(
            f"Expected one 'By leveling up' header, got {len(table_headers)}"
        )

    if len(table_rows) < 1:
        raise RuntimeError(
            f"Expected multiple 'By leveling up' rows, got {len(table_rows)}"
        )

    if len(table_footer) != 1:
        raise RuntimeError(
            f"Expected one 'By leveling up' footer, got {len(table_footer)}"
        )

    first_row = table_rows[0].split("|")

    header = ["Move", "Type", "Power", "Accuracy", "PP"]  # The fix columns

    # Check if the second column of interest contains a move or a level.
    # If its a level, then there is a difference between game versions.
    if first_row[2].isalpha():
        column_count = 6
        header.insert(0, "Level")
    else:
        column_count = 7
        markdown_header_elems = table_headers[0].removesuffix("}}").split("|")
        header.insert(0, markdown_header_elems[-1])  # Game version
        header.insert(0, markdown_header_elems[-2])  # Game version

    rows = []
    for row in table_rows:
        clean_row = row.removesuffix("}}").split("|")[1 : 1 + column_count]
        rows.append(clean_row)

    table = [header] + rows

    column_count = len(table[0])
    for row in table:
        if len(row) != column_count:
            raise ValueError("Mismatch of element count of rows in table.")

    return table


def main():
    print("Collecting Gen 1 learnset articles.")

    article_urls = get_learnset_article_urls()
    if len(article_urls) != 151:
        raise RuntimeError(f"Expected URLs for 151 Pokemon. Found {len(article_urls)}.")

    print(f"Found {len(article_urls)} article URLs.")

    pkmn_entries = []
    for url in tqdm(article_urls):
        markdown_source = get_wiki_article_markdown_source(url=url)

        ndex, pkmn = get_pkmn(markdown_source)
        table = get_level_learnset(markdown_source)

        entry = {
            "ndex": ndex,
            "pokemon": pkmn,
            "by_leveling_up": table,
        }
        pkmn_entries.append(entry)

    result_json_path = Path("learnset.json")
    with result_json_path.open("w", encoding="utf-8") as f:
        json_str = json.dumps(pkmn_entries, indent=4, ensure_ascii=False)
        f.write(json_str)

    print(f"Written JSON file to {result_json_path.absolute()}")


if __name__ == "__main__":
    main()
