"""Collects the learnsets from Bulbapedia.

The learnset category page contains the generation learnset (sub)category pages.

The generation learnset (sub)category pages contain the learnset article pages.
Maximum 200 article pages can be listed at a time, therefore the (sub)category
can have multiple pages.

The learnset article page contains the learnset of a specific pokemon for the
specific generation.

Category:Pokémon learnsets
https://bulbapedia.bulbagarden.net/wiki/Category:Pok%C3%A9mon_learnsets

Category:Pokémon learnsets (Generation I)
https://bulbapedia.bulbagarden.net/wiki/Category:Pok%C3%A9mon_learnsets_(Generation_I)
"""

import json
from multiprocessing import Pool
from pathlib import Path
from urllib.parse import urljoin

import bs4
import requests
from bs4 import BeautifulSoup
from tqdm import tqdm


def get_learnset_article_urls(category_url: str) -> list[str]:
    """Returns the article URLs from the generation learnset (sub)category.

    The `category_url` is the first page of the generation learnset (sub)category page.
    E.g. https://bulbapedia.bulbagarden.net/wiki/Category:Pok%C3%A9mon_learnsets_(Generation_II)
    """

    categ_page_urls = [category_url]
    article_urls = []

    while categ_page_urls:
        curr_category_url = categ_page_urls.pop(0)

        response = requests.get(curr_category_url)
        if response.status_code != 200:
            raise Exception(
                f"Failed fetching page. Status code: {response.status_code}"
            )

        soup = BeautifulSoup(response.content, "html.parser")

        for a in soup.find_all("a"):  # Links
            if not a.contents:
                continue  # No human-readable text

            text: str = a.contents[0]
            relative_url: str = a["href"]
            full_url = urljoin("https://bulbapedia.bulbagarden.net", relative_url)

            if text == "next page":
                if full_url not in categ_page_urls:
                    # Only need one "next page" link per page. Two is present.
                    categ_page_urls.append(full_url)

            if " (Pokémon)/Generation " in text:
                article_urls.append(full_url)

    return article_urls


def article_url_to_source_url(url: str) -> str:
    """Returns the Bulbapedia source page url from the article page url.

    Seems to work with all Bulbapedia pages.

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


def get_pokemon_name_and_ndex(markdown_source: str) -> tuple[str, str]:
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


def get_learnset_leveling_up(markdown_source: str) -> list[list[str]]:
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

        if row.startswith("{{learnlist/levelf"):
            table_footer.append(row)
            continue

        # Wiki macro differs when the table contains one level data or two.
        # With same moves, its leveln, where n is an Arab number.
        # With separate moves, its levelN, where N is a Roman number.
        if row.startswith("{{learnlist/level"):
            table_rows.append(row)
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
    if first_row[2].isnumeric():
        column_count = 7
        markdown_header_elems = table_headers[0].removesuffix("}}").split("|")
        header.insert(0, markdown_header_elems[-1])  # Game version
        header.insert(0, markdown_header_elems[-2])  # Game version
    else:
        column_count = 6
        header.insert(0, "Level")

    rows = []
    for row in table_rows:
        clean_row = row.removesuffix("}}").split("|")[1 : 1 + column_count]
        if len(clean_row) != column_count:
            raise ValueError(
                "Failed to retrieve row from markdown row. ",
                f"Expected {column_count} elements, got {len(clean_row)}.\n",
                f"{row}\n",
                f"{clean_row}\n",
            )
        rows.append(clean_row)

    table = [header] + rows

    column_count = len(table[0])
    for row in table:
        if len(row) != column_count:
            raise ValueError("Mismatch of element count of rows in table.")

    return table


def norm_learnset_table(table: list[list[str]]) -> list[list[str]]:
    """If present, splits the Level column into RGB and Y columns."""
    raise NotImplementedError("Only works for Gen 1")

    if table[0][0] == "RGB" and table[0][1] == "Y":
        return table

    if table[0][0] != "Level":
        raise ValueError(f"Got unexpected header in learnset table: {table[0]}")

    normed_header = ["RGB", "Y"] + table[0][1:]

    normed_rows = []
    for row in table[1:]:
        normed_row = [row[0]] + row
        normed_rows.append(normed_row)

    normed_table = normed_header + normed_rows

    return normed_table


def scrape_gen_learnset(gen: str, category_url: str) -> None:
    """Scrapes the learnsets into a JSON file."""

    print(f"\nCollecting Gen {gen} article URLs from {category_url}")

    article_urls = get_learnset_article_urls(category_url)

    print(f"Found {len(article_urls)} learnset articles.")

    print("Downloading article Markdown sources:")
    with Pool(8) as p:
        markdown_sources = list(
            tqdm(
                p.imap(get_wiki_article_markdown_source, article_urls),
                total=len(article_urls),
            )
        )

    # Bulbapedia to Smogon names
    smogon_names = {
        "Nidoran♀": "Nidoran-F",
        "Nidoran♂": "Nidoran-M",
    }

    pkmn_entries = []
    for markdown_source in markdown_sources:
        ndex, pokemon = get_pokemon_name_and_ndex(markdown_source)
        if pokemon in smogon_names:
            pokemon = smogon_names[pokemon]

        table = get_learnset_leveling_up(markdown_source)
        table = [row[:-4] for row in table]  # Remove Type, Powr, Acc and PP
        # normed_table = norm_learnset_table(table)

        entry = {
            "ndex": ndex.strip("0"),
            "pokemon": pokemon,
            "by_leveling_up": table,
        }
        pkmn_entries.append(entry)

    script_dir = Path(__file__).parent
    dst_dir = Path(script_dir.parent, "data")
    if not dst_dir.is_dir():
        print(f"Creating dir at {dst_dir}")
        dst_dir.mkdir()

    dst_path = Path(dst_dir, f"gen{gen}_learnsets.json")
    with dst_path.open("w", encoding="utf-8") as f:
        json_str = json.dumps(pkmn_entries, indent=4, ensure_ascii=False)
        f.write(json_str)

    print(f"Wrote Gen {gen} learnset to {dst_path}")


def main():
    gen_learnset_categ_urls = [
        (
            "i",
            "https://bulbapedia.bulbagarden.net/wiki/Category:Pok%C3%A9mon_learnsets_(Generation_I)",
        ),
        (
            "ii",
            "https://bulbapedia.bulbagarden.net/wiki/Category:Pok%C3%A9mon_learnsets_(Generation_II)",
        ),
    ]

    for gen, url in gen_learnset_categ_urls:
        scrape_gen_learnset(gen, url)


if __name__ == "__main__":
    main()
