"""Debug functionality to be used with CodeLLDB and the Debug console.

Module needs to be imported in the launch.json:
```
"initCommands": [
    "command script import ${workspaceFolder}/path/to/module"
]
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

matplotlib.use("agg")


def rust_to_numpy_dtype(rust_type: str):
    """Returns the numpy dtype alias from the Rust type.

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
    result = rust_to_numpy_type[rust_type]
    return result


def get_color_info(type_description: str) -> tuple[str, str]:
    """Returns the color space and elem type from the type description.

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

    Input:
    ```
    struct alloc::vec::Vec<u8, alloc::alloc::Global> {
        buf: alloc::raw_vec::RawVec<u8, alloc::alloc::Global>,
        len: usize
    }
    ```

    Output:
    ```
    u8
    ```
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


def show():
    """Shows all open pyplot figures in a VSCode tab."""
    image_bytes = io.BytesIO()
    plt.savefig(image_bytes, format="png", bbox_inches="tight")
    bytes = base64.b64encode(image_bytes.getvalue()).decode("utf-8")
    document = f'<html><img src="data:image/png;base64,{bytes}"></html>'
    debugger.display_html(document, position=2)


def plot_img(image):
    """Plots an image.DynamicImage or image.ImageBuffer instance.

    Only Luma, Rgb, Rgba color spaces are supported.
    """
    image = debugger.unwrap(image)

    image_type = str(image.type)
    if "DynamicImage" in image_type:
        image_buffer = image.GetChildAtIndex(0)
    else:
        image_buffer = image

    color_space, rust_type = get_color_info(image_type)
    numpy_dtype = rust_to_numpy_dtype(rust_type)
    elem_size = np.dtype(numpy_dtype).itemsize  # The array elements in bytes

    width = image_buffer.GetChildMemberWithName("width").GetValueAsUnsigned()
    height = image_buffer.GetChildMemberWithName("height").GetValueAsUnsigned()
    data = image_buffer.GetChildMemberWithName("data")
    addr = data.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()

    if color_space.lower() == "luma":
        shape = (height, width)
        byte_count = height * width * elem_size
    elif color_space.lower() == "rgb":
        shape = (height, width, 3)
        byte_count = height * width * 3 * elem_size
    elif color_space.lower() == "rgba":
        shape = (height, width, 4)
        byte_count = height * width * 4 * elem_size
    else:
        raise RuntimeError(f"Color space {color_space} is unsupported.")

    data = lldb.process.ReadMemory(addr, byte_count, lldb.SBError())
    data = np.frombuffer(data, dtype=numpy_dtype).reshape(shape)

    # cmap is ignored for RGB(A) data
    plt.imshow(data, cmap="gist_gray", interpolation="nearest")
    show()
    print("width: {}".format(width))
    print("height: {}".format(height))
    print("color space: {}".format(color_space))
    print("item type: {}".format(rust_type))


def plot_vec(image_vec, width, height, color_space):
    """Plots an an image from a Vec<_>."""
    image_vec = debugger.unwrap(image_vec)
    image_addr = image_vec.GetChildAtIndex(0).AddressOf().GetValueAsUnsigned()

    rust_type = get_vec_type(image_vec.type)
    numpy_dtype = rust_to_numpy_dtype(rust_type)
    elem_size = np.dtype(numpy_dtype).itemsize  # Bytes

    if color_space.lower() == "luma":
        shape = (height, width)
        byte_count = height * width * elem_size
    elif color_space.lower() == "rgb":
        shape = (height, width, 3)
        byte_count = height * width * 3 * elem_size
    elif color_space.lower() == "rgba":
        shape = (height, width, 4)
        byte_count = height * width * 4 * elem_size
    else:
        raise RuntimeError(f"Color space {color_space} is unsupported.")

    data = lldb.process.ReadMemory(image_addr, byte_count, lldb.SBError())
    data = np.frombuffer(data, dtype=numpy_dtype).reshape(shape)

    plt.imshow(data, cmap="gist_gray", interpolation="nearest")
    show()
    print("width: {}".format(width))
    print("height: {}".format(height))
    print("color space: {}".format(color_space))
    print("item type: {}".format(rust_type))


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
