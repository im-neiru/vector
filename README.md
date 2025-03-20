# Vector

Under development 🏗️

## Prerequisites

### Rust

- **Rust Version:** 1.85.0
- **Edition:** 2024
Ensure you have the specific Rust toolchain installed. You can install it from the [official Rust website](https://www.rust-lang.org/tools/install).

### Vulkan SDK

- **Vulkan SDK:**
For graphics rendering, install the LunarG Vulkan SDK:

- **Windows:**
  Download and install the LunarG Vulkan SDK from [LunarG's website](https://vulkan.lunarg.com/). Make sure to select version **1.3.261.0** or a later compatible release.

- **Ubuntu:**
  You can install the Vulkan loader library using your package manager:

  ```shell
  sudo apt update
  sudo apt install libvulkan1


### Slang

- **Slang Version:** 2024.17-1\
  Ensure you have Slang version **2024.17-1** installed. This library is required for shader management and compilation. Follow the [Slang releases](https://github.com/shader-slang/slang/releases) for more details.

## Building the Project

**Set Environment Variables:**

For linking Slang, set the `SLANG_LIB` environment variable to point to the directory containing `slang.lib`.

**Windows Example:**

```shell
set SLANG_LIB=C:\path\to\your\lib
