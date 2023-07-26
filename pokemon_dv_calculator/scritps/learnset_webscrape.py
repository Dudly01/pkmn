"""Collects the "By leveling up" learnset from Bulbapedia."""

from urllib.parse import urljoin

import bs4
import requests
from bs4 import BeautifulSoup


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


def get_wiki_article_markup_source(url: str) -> str:
    """Returns the markup source of the Bulbapedia article as a string."""

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


def main():
    article_urls = get_learnset_article_urls()

    if len(article_urls) != 151:
        raise RuntimeError(f"Expected URLs for 151 Pokemon. Found {len(article_urls)}.")

    for url in article_urls:
        markup_source = get_wiki_article_markup_source(url=url)
        print(markup_source)
        break


if __name__ == "__main__":
    main()
