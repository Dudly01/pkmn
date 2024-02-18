"""Demo script for playing with plotly.

Sources:
- https://plotly.com/python/imshow/
- https://stackoverflow.com/questions/37683147
"""


from pathlib import Path

import numpy as np
import plotly
import plotly.express as px


def main():
    img = np.arange(15**2).reshape((15, 15))
    fig = px.imshow(img)

    fig.update_layout(
        dragmode="pan",  # Default selection no longer zoom
        xaxis={"mirror": "allticks", "side": "top"},
    )
    config = {
        "scrollZoom": True,
        "displaylogo": False,
    }

    # Uses "old" approach
    div = plotly.offline.plot(
        fig, config=config, include_plotlyjs=False, output_type="div"
    )
    document = f'<html><script src="https://cdn.plot.ly/plotly-latest.min.js"></script>{div}</html>'
    Path("plotly.html").write_text(document)

    # Uses "new" approach
    div_script = plotly.io.to_html(fig, config, include_plotlyjs=False, full_html=False)
    document = f'<html><script src="https://cdn.plot.ly/plotly-latest.min.js"></script>{div_script}</html>'
    Path("plotly1.html").write_text(document)

    # Returns a full standalone HTML
    html = plotly.io.to_html(fig, config)
    Path("plotly2.html").write_text(html)


if __name__ == "__main__":
    main()
