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


def plot_rgb8_dyn_img(image_dynamic):
    """Plots a image.DynamicImage instance."""
    image_dynamic = debugger.unwrap(image_dynamic)
    image_buffer = image_dynamic.GetChildAtIndex(0)

    width = image_buffer.GetChildMemberWithName("width").GetValueAsUnsigned()
    height = image_buffer.GetChildMemberWithName("height").GetValueAsUnsigned()
    data = image_buffer.GetChildMemberWithName("data")
    addr = data.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()

    data = lldb.process.ReadMemory(addr, int(height * width) * 3, lldb.SBError())
    data = np.frombuffer(data, dtype=np.uint8).reshape((height, width, 3))
    plt.imshow(data, interpolation="nearest")
    show()


def plot_rgb8_img_buff(image_buffer):
    """Plots an image.ImageBuffer instance."""
    image_buffer = debugger.unwrap(image_buffer)

    width = image_buffer.GetChildMemberWithName("width").GetValueAsUnsigned()
    height = image_buffer.GetChildMemberWithName("height").GetValueAsUnsigned()
    data = image_buffer.GetChildMemberWithName("data")
    addr = data.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()

    data = lldb.process.ReadMemory(addr, int(height * width) * 3, lldb.SBError())
    data = np.frombuffer(data, dtype=np.uint8).reshape((height, width, 3))
    plt.imshow(data, interpolation="nearest")
    show()


def plot_rgb8_vec(image_vec, xdim, ydim):
    """Plots an RGB8 image from a vector and dimensions."""
    image_vec = debugger.unwrap(image_vec)
    image_addr = image_vec.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()
    
    data = lldb.process.ReadMemory(image_addr, int(xdim * ydim) * 3, lldb.SBError())
    data = np.frombuffer(data, dtype=np.uint8).reshape((ydim, xdim, 3))
    plt.imshow(data, interpolation="nearest")
    show()


def plot_rgb8_ptr(image_ptr, xdim, ydim):
    """Plots an RGB8 image from a pointer and dimensions."""
    image_ptr = debugger.unwrap(image_ptr)
    image_addr = image_ptr.GetValueAsUnsigned()

    data = lldb.process.ReadMemory(image_addr, int(xdim * ydim) * 3, lldb.SBError())
    data = np.frombuffer(data, dtype=np.uint8).reshape((ydim, xdim, 3))
    plt.imshow(data, interpolation="nearest")
    show()


def info(value):
    """Writes info about the codelldb.value.Value into the terminal."""

    if str(type(value)) == "<class 'codelldb.value.Value'>":
        print("Got <class 'codelldb.value.Value'> as input")

    sb_value = debugger.unwrap(value)  # Extract lldb.SBValue

    print("- lldb.SBValue:")
    print("  {}".format(sb_value))

    print("- type:")
    print("  {}".format(sb_value.type))

    print("- TypeIsPointerType():")
    print("  {}".format(sb_value.TypeIsPointerType()))

    print("- GetValueAsUnsigned() (hex):")
    print(
        "  {} ({})".format(
            sb_value.GetValueAsUnsigned(),
            hex(sb_value.GetValueAsUnsigned()),
        )
    )

    print("- AddressOf().GetValueAsUnsigned() (hex):")
    print(
        "  {} ({})".format(
            sb_value.AddressOf().GetValueAsUnsigned(),
            hex(sb_value.AddressOf().GetValueAsUnsigned()),
        )
    )

    print("- num_children:")
    print("  {}".format(sb_value.num_children))

    print("- GetChildAtIndex(0):")
    print("  {}".format(sb_value.GetChildAtIndex(0)))
    
    print("- GetChildAtIndex(0).type:")
    print("  {}".format(sb_value.GetChildAtIndex(0).type))

    print("- GetChildAtIndex(0).TypeIsPointerType():")
    print("  {}".format(sb_value.GetChildAtIndex(0).TypeIsPointerType()))

    print("- GetChildAtIndex(0).GetValueAsUnsigned() (hex):")
    print(
        "  {} ({})".format(
            sb_value.GetChildAtIndex(0).GetValueAsUnsigned(),
            hex(sb_value.GetChildAtIndex(0).GetValueAsUnsigned()),
        )
    )

    print("- GetChildAtIndex(0).AddressOf().GetValueAsUnsigned() (hex):")
    print(
        "  {} ({})".format(
            sb_value.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned(),
            hex(sb_value.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()),
        )
    )
