"""Creates the index.html from the contents .md and the frame .html file."""

import subprocess
from pathlib import Path

from bs4 import BeautifulSoup, NavigableString


def insert_markdown_content(
    content_path: Path,
    html_frame_path: Path,
    out_dir: Path,
):

    # Convert markdown to html with pandoc
    markdown_dir = content_path.parent
    command = "pandoc -f markdown -t html index.md -o pandoc_output.html --citeproc --highlight-style zenburn"
    subprocess.run(command, shell=True, check=True, cwd=markdown_dir)

    # Get pandoc output soup
    pandoc_output_path = markdown_dir / "pandoc_output.html"
    pandoc_output_text = pandoc_output_path.read_text()
    pandoc_output_soup = BeautifulSoup(pandoc_output_text, "html.parser")

    # Format external links
    for link in pandoc_output_soup.find_all("a"):
        href: str = link.get("href")
        if href.startswith("http"):
            link["target"] = "_blank"  # Open link in new tab
            link.append(
                pandoc_output_soup.new_tag(
                    "i", attrs={"class": "fa fa-external-link", "aria_hidden": "true"}
                )  # External link favicon
            )

    # Remove aria-hidden="true" attribute created by pandoc
    for tag in pandoc_output_soup.find_all(attrs={"aria-hidden": "true"}):
        del tag["aria-hidden"]

    # Find TOC
    toc = pandoc_output_soup.find("ul")
    if toc is None:
        raise ValueError("Did not find 'ul' element of the TOC")

    # Find content
    content_start = pandoc_output_soup.find("h1")
    if content_start is None:
        raise ValueError("Did not find start of the content.")

    content_elems = [content_start]
    for sibling in content_start.next_siblings:
        if isinstance(sibling, NavigableString):
            if str(sibling) == "\n":
                continue  # We do not want top-level \n
        content_elems.append(sibling)

    # content_html_path = blogpost_dir / "test_content.html"
    # with content_html_path.open("w") as f:
    #     f.writelines((repr(elem) for elem in content_elems))

    # Get frame soup
    post_frame_text = html_frame_path.read_text()
    frame_soup = BeautifulSoup(post_frame_text, "html.parser")

    # Insert TOC
    toc_dst = frame_soup.find("section", class_="document-toc")
    if toc_dst is None:
        raise ValueError("Did not find TOC destination element")
    toc_dst.append(toc)

    # Insert content
    content_dst = frame_soup.find("div", class_="blog-post")
    if content_dst is None:
        raise ValueError("Did not find content destination div elem")
    content_dst = content_dst.find("article")
    if content_dst is None:
        raise ValueError("Did not find content destination article elem")
    content_dst.extend(content_elems)

    # Save frame as blogpost
    post_html_path = out_dir / "index.html"
    post_html_path.write_text(str(frame_soup), "utf-8")  # soup.prettify() could be used

    return


def main():
    project_root = Path(__file__).parents[2]
    insert_markdown_content(
        content_path=project_root / "net" / "index.md",
        html_frame_path=project_root / "net" / "scripts" / "index_frame.html",
        out_dir=project_root / "net" / "static",
    )


if __name__ == "__main__":
    main()
