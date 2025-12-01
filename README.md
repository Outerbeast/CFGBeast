# CFGBeast
![alt text](https://github.com/Outerbeast/CFGBeast/blob/main/preview.png?raw=true)

BSP map configurator

Quick and easy automatic CFG file generator for BSP files in one go, with preset CVars that can be selected

## Installation
- Download the application from the [Releases](https://github.com/Outerbeast/CFGBeast/releases/) section
- Place the executable file into your folder containing the .bsp files
- Launch the executable for initial setup, this will search for your Sven Co-op game install.

## Usage
Input your CVars into the textbox by selecting preset CVars from the list, or putting them in manually into the box.

Select what you want to with the input CVars with the following buttons:
- `Create`: creates new `.cfg` files with the CVars. If the CFG files already exists, the previous files will be overwritten.
- `Add`: adds the CVars to the existing CFG files.
- `Remove`: removes the selected CVars if they exist in the CFG files.
- `Delete`: This will delete the CFG files from the folder.

The list of BSPs selected are shown in the list on the left - you may uncheck BSPs so CFG generation/deletion is skipped.

You can also toggle skill CFG generation using the checkbox. This will show all the relevant skill CVars and will generate `*_skl.cfg` files.

### Quick Create

- Dragging a .cfg file onto the CFGBeast executable will copy that cfg and generate CFG files for all the BSPs automatically.
  - MOTD files (`*_motd.txt`) will do the same thing

# Building from source

## Prerequisites

1️⃣ Install Rust
- Visit [https://rustup.rs](https://rustup.rs) and download the Windows installer.
- Run it and accept the defaults (this installs `cargo`, `rustc`, and `rustup`).
- Close and reopen any terminal/PowerShell windows after installation.

---

2️⃣ Install Windows Build Tools
The GUI uses the Windows API, so you need the C++ build toolchain:

**Option A (Recommended)**  
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).  
- During installation, select **"Desktop development with C++"**.

**Option B**  
- Add the MSVC target via:
```powershell
rustup target add x86_64-pc-windows-msvc
```

## Build instructions
1. Download or clone the repository:

```cmd
git clone https://github.com/Outerbeast/CFGBeast.git
cd CFGBeast
```

2. Run the build script:
- Double-click build.cmd or run it manually:
```cmd
build.cmd
```

The executable will be generated in the current directory.

# Feedback & Issues
If you have feedback or encounter issues, please open an issue on [GitHub Issues](https://github.com/Outerbeast/CFGBeast/issues).


### Credits
- Outerbeast - Author
- Garompa - Testing and feedback