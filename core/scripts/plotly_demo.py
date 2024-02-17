"""Demo script for playing with plotly."""


import plotly.express as px
import numpy as np


def main():
    img = np.arange(15**2).reshape((15, 15))
    fig = px.imshow(img)
    fig.show()


if __name__ == "__main__":
    main()
