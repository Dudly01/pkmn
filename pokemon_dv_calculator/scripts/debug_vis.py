"""Debug functionality to be used with CodeLLDB and the Debug console.

Module needs to be imported in the launch.json.

From the debug console:
```
?/py debug_vis.plot_rgb8($img_addr, $width, $height)
```

From the conditional breakpoint:
/py debug_vis.plot_rgb8($img_addr, $width, $height) if True else False
"""

import base64
import io

import debugger  # CodeLLDB auto import
import lldb  # LLDB auto import
import matplotlib
import matplotlib.pyplot as plt
import numpy as np

matplotlib.use("agg")


def show():
    """Shows all open pyplot figures in a VSCode tab."""
    image_bytes = io.BytesIO()
    plt.savefig(image_bytes, format="png", bbox_inches="tight")
    bytes = base64.b64encode(image_bytes.getvalue()).decode("utf-8")
    document = f'<html><img src="data:image/png;base64,{bytes}"></html>'
    debugger.display_html(document, position=2)


def plot_rgb8(image, xdim, ydim):
    """Plots an RGB8 image from its pointer and dimensions."""
    image = debugger.unwrap(image)

    if image.TypeIsPointerType():
        image_addr = image.GetValueAsUnsigned()
    else:
        image_addr = image.AddressOf().GetValueAsUnsigned()
    data = lldb.process.ReadMemory(image_addr, int(xdim * ydim) * 3, lldb.SBError())
    data = np.frombuffer(data, dtype=np.uint8).reshape((ydim, xdim, 3))
    plt.imshow(data, interpolation="nearest")

    image_bytes = io.BytesIO()
    plt.savefig(image_bytes, format="png", bbox_inches="tight")
    bytes = base64.b64encode(image_bytes.getvalue()).decode("utf-8")
    document = f'<html><img src="data:image/png;base64,{bytes}"></html>'
    debugger.display_html(document, position=2)


def info(value):
    """Writes info about the codelldb.value.Value into the terminal."""
    sb_value = debugger.unwrap(value)  # Extract lldb.SBValue

    print("lldb.SBValue:")
    print(" - {}".format(sb_value))

    print("type:")
    print(" - {}".format(sb_value.type))

    print("TypeIsPointerType():")
    print(" - {}".format(sb_value.TypeIsPointerType()))

    print("GetValueAsUnsigned() (hex):")
    print(
        " - {} ({})".format(
            sb_value.GetValueAsUnsigned(),
            hex(sb_value.GetValueAsUnsigned()),
        )
    )

    print("AddressOf().GetValueAsUnsigned() (hex):")
    print(
        " - {} ({})".format(
            sb_value.AddressOf().GetValueAsUnsigned(),
            hex(sb_value.AddressOf().GetValueAsUnsigned()),
        )
    )

    print("num_children:")
    print(" - {}".format(sb_value.num_children))

    print("GetChildAtIndex(0).TypeIsPointerType()")
    print(" - {}".format(sb_value.GetChildAtIndex(0).TypeIsPointerType()))

    print("GetChildAtIndex(0).GetValueAsUnsigned() (hex)")
    print(
        " - {} ({})".format(
            sb_value.GetChildAtIndex(0).GetValueAsUnsigned(),
            hex(sb_value.GetChildAtIndex(0).GetValueAsUnsigned()),
        )
    )

    print("GetChildAtIndex(0).AddressOf().GetValueAsUnsigned() (hex)")
    print(
        " - {} ({})".format(
            sb_value.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned(),
            hex(sb_value.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()),
        )
    )
