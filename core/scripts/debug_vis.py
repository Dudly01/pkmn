"""Debug functionality to be used with CodeLLDB and the Debug console.

Module needs to be imported in the launch.json:
```
"initCommands": [
    "command script import ${workspaceFolder}/path/to/module"
]
```

The numpy and matplotlib packages need to be installed to the Python
bundles with CodeLLDB. Use the `LLDB: Command Prompt` command in VSCode
followed by:

```
pip install numpy matplotlib
```

The function can be called from the debug console:
```
?/py debug_vis.plot_rgb8($img_addr, $width, $height)
```
"""

import base64
import io

import debugger  # CodeLLDB auto import
import lldb  # LLDB auto import
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import plotly
import plotly.express as px
import plotly.graph_objects as go

matplotlib.use("agg")


def get_rust_numpy_equiv(rust_type: str):
    """Returns the numpy dtype equivalent of a primitive Rust type.

    Example:
        "u8" -> np.uint8
    """
    rust_to_numpy_type = {
        "u8": np.uint8,
        "u16": np.uint16,
        "u32": np.uint32,
        "u64": np.uint64,
        "i8": np.int8,
        "i16": np.int16,
        "i32": np.int32,
        "i64": np.int64,
        "f32": np.float32,
        "f64": np.float64,
    }

    if rust_type not in rust_to_numpy_type:
        raise ValueError(f"The Rust type {rust_type} has no numpy dtype equivalent.")

    result = rust_to_numpy_type[rust_type]
    return result


def get_image_color_info(type_description: str) -> tuple[str, str]:
    """Returns the color space and elem type from the Image type description.

    Example:
        [...]image::color::LumaA<u16>[...] -> (LumaA, u16),
        where [...] means text without the substring "image::color::".
    """
    type_description = str(type_description)

    target_substring = "image::color::"
    pos = type_description.find(target_substring)
    if pos == -1:
        raise RuntimeError("image::color:: not found within description")
    start = pos + len(target_substring)

    mid = type_description.find("<", start)
    if mid == -1:
        raise RuntimeError("type opening < not found within description")

    end = type_description.find(">", mid)
    if end == -1:
        raise RuntimeError("type closing > not found within description")

    color_space = type_description[start:mid]
    elem_type = type_description[mid + 1 : end]

    return color_space, elem_type


def get_vec_type(type_description: str) -> str:
    """Returns the type of the Vec.

    Example:
        [...]alloc::vec::Vec<u8,[...] -> u8,
        where [...] means text without the substring "alloc::vec::".
    """
    type_description = str(type_description)

    target_substring = "alloc::vec::Vec<"
    pos = type_description.find(target_substring)
    if pos == -1:
        raise RuntimeError(f"{target_substring} not found within description")
    start = pos + len(target_substring)

    end = type_description.find(",", start)
    if end == -1:
        raise RuntimeError("Generic separating ',' not found.")

    elem_type = type_description[start:end]

    return elem_type


def get_channel_count(color_space: str) -> int:
    """Returns the number of channels for the given color space.

    Only luma, rgb and rgba color spaces are supported, due to the pyplot dependency.
    The "color_space" argument is converted to a string and lowercased.
    """
    color_space = str(color_space)

    channel_counts = {
        "luma": 1,
        "rgb": 3,
        "rgba": 4,
    }

    if color_space.lower() not in channel_counts:
        raise ValueError(f"Color space {color_space} is unsupported.")

    channel_count = channel_counts[color_space.lower()]
    return channel_count


def show():
    """Shows all open pyplot figures in a VSCode tab."""
    image_bytes = io.BytesIO()
    plt.savefig(image_bytes, format="png", bbox_inches="tight")
    bytes = base64.b64encode(image_bytes.getvalue()).decode("utf-8")
    document = f'<html><img src="data:image/png;base64,{bytes}"></html>'
    debugger.display_html(document, position=2)


def show_plotly(fig: go.Figure):
    """Shows the Plotly Figure in a VSCode tab."""

    # html = plotly.io.to_html(fig)  # Full html is way bigger in size

    div = plotly.io.to_html(fig, include_plotlyjs=False, full_html=False)
    html = f'<html><script src="https://cdn.plot.ly/plotly-latest.min.js"></script>{div}</html>'

    debugger.display_html(html, position=2)


def plot_roi(roi):
    """Plots a crate::roi::Roi instance."""
    roi = debugger.unwrap(roi)

    image = roi.GetChildMemberWithName("img")
    position = roi.GetChildMemberWithName("pos")

    image_type = str(image.type)
    image_buffer = image.GetChildAtIndex(0) if "DynamicImage" in image_type else image

    color_space, rust_type = get_image_color_info(image_type)
    numpy_dtype = get_rust_numpy_equiv(rust_type)
    elem_size = np.dtype(numpy_dtype).itemsize  # The array elements in bytes

    width = image_buffer.GetChildMemberWithName("width").GetValueAsUnsigned()
    height = image_buffer.GetChildMemberWithName("height").GetValueAsUnsigned()
    data = image_buffer.GetChildMemberWithName("data")
    addr = data.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()

    channel_count = get_channel_count(color_space.lower())

    shape = (height, width, channel_count)
    byte_count = height * width * channel_count * elem_size

    data = lldb.process.ReadMemory(addr, byte_count, lldb.SBError())
    data = np.frombuffer(data, dtype=numpy_dtype).reshape(shape)

    x_pos = position.GetChildMemberWithName("x").GetValueAsUnsigned()
    y_pos = position.GetChildMemberWithName("y").GetValueAsUnsigned()
    width_pos = position.GetChildMemberWithName("width").GetValueAsUnsigned()
    height_pos = position.GetChildMemberWithName("height").GetValueAsUnsigned()

    data_of_interest = data[y_pos : y_pos + height_pos, x_pos : x_pos + width_pos]

    fig = px.imshow(data_of_interest)
    show_plotly(fig)
    print("width: {}".format(width_pos))
    print("height: {}".format(height_pos))
    print("color space: {}".format(color_space))
    print("item type: {}".format(rust_type))


def plot_img(image):
    """Plots an image::DynamicImage or image::ImageBuffer instance.

    Only Luma, Rgb, Rgba color spaces are supported.
    """
    image = debugger.unwrap(image)

    image_type = str(image.type)
    image_buffer = image.GetChildAtIndex(0) if "DynamicImage" in image_type else image

    color_space, rust_type = get_image_color_info(image_type)
    numpy_dtype = get_rust_numpy_equiv(rust_type)
    elem_size = np.dtype(numpy_dtype).itemsize  # The array elements in bytes

    width = image_buffer.GetChildMemberWithName("width").GetValueAsUnsigned()
    height = image_buffer.GetChildMemberWithName("height").GetValueAsUnsigned()
    data = image_buffer.GetChildMemberWithName("data")
    addr = data.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()

    channel_count = get_channel_count(color_space.lower())

    shape = (height, width, channel_count)
    byte_count = height * width * channel_count * elem_size

    data = lldb.process.ReadMemory(addr, byte_count, lldb.SBError())
    data = np.frombuffer(data, dtype=numpy_dtype).reshape(shape)

    fig = px.imshow(data)
    show_plotly(fig)
    print("width: {}".format(width))
    print("height: {}".format(height))
    print("color space: {}".format(color_space))
    print("item type: {}".format(rust_type))


def plot_vec(vec, width, height, color_space):
    """Plots an an image from a Vec<_>."""
    vec = debugger.unwrap(vec)
    image_addr = vec.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()

    rust_type = get_vec_type(vec.type)
    numpy_dtype = get_rust_numpy_equiv(rust_type)
    elem_size = np.dtype(numpy_dtype).itemsize  # Bytes

    channel_count = get_channel_count(color_space.lower())

    shape = (height, width, channel_count)
    byte_count = height * width * channel_count * elem_size

    data = lldb.process.ReadMemory(image_addr, byte_count, lldb.SBError())
    data = np.frombuffer(data, dtype=numpy_dtype).reshape(shape)

    plt.imshow(data, cmap="gist_gray", interpolation="nearest")
    show()
    print("width: {}".format(width))
    print("height: {}".format(height))
    print("color space: {}".format(color_space))
    print("item type: {}".format(rust_type))


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
