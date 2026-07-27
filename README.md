# CFGBeast

![alt text](https://github.com/Outerbeast/CFGBeast/blob/main/preview.jpg?raw=true)

BSP map configurator

Quick and easy automatic CFG file generator, resource and materials replacer for BSP files in one go, with preset CVars that can be selected

## Installation

- Download the application from the [Releases](https://github.com/Outerbeast/CFGBeast/releases/) section
- Place the executable file into your folder containing the .bsp files
- Launch the executable for initial setup, this will search for your Sven Co-op game install.

## Usage

You can switch between the different tabs to select between 3 modes:
- CFG Generator
- Resource Replacer
- Materials Replacer

### CFG Generator

![alt text](https://github.com/Outerbeast/CFGBeast/blob/main/demo_cfg.png?raw=true)

Input your CVars into the textbox by selecting preset CVars from the list, dragging in an exiting CFG file into the box, or putting them in manually into the box.

Select what you want to with the input CVars with the following buttons:

- `Load CFG`: loads CVars from an existing `.cfg` file into the textbox.
- `Create`: creates new `.cfg` files with the CVars. If the CFG files already exists, the previous files will be overwritten.
- `Add`: adds the CVars to the existing CFG files.
- `Remove`: removes the selected CVars if they exist in the CFG files.
- `Delete`: This will delete the CFG files from the folder.
- `Change Folder`: Changes the current BSP folder

The list of BSPs selected are shown in the list on the left - you may uncheck BSPs so CFG generation/deletion is skipped.

You can also toggle skill CFG generation using the checkbox. This will show all the relevant skill CVars and will generate `*_skl.cfg` files.

### Resource Replacer

![alt text](https://github.com/Outerbeast/CFGBeast/blob/main/demo_resourcereplacer.png?raw=true)

Creating replacements:
1. Add replacement pairs using one of two methods:
- Manually: click Add, pick an original file then a replacement file. Both must be the same type (model→model or sound→sound). Absolute paths are automatically truncated to relative game paths (e.g. `C:/Sven Co-op/svencoop/models/player.mdl` becomes `models/player.mdl`).
- Load existing: click Load Replacements to import an existing .gmr/.gsr file into the table.
2. Select a row and click Remove to delete it. Click Remove with no row selected to clear all.
3. Click Create to write the `.gmr` and/or `.gsr` files. A save dialog will prompt for the output filename (no extension needed — `.gmr`/`.gsr` are appended automatically).
Supported file types:
- Models (`.gmr`): `.mdl`, `.spr`
- Sounds (`.gsr`): `.wav`, `.mp3`, `.ogg`, `.aiff`, `.flac`, and others
Quick Create:
Dragging a `.gmr` or `.gsr` file onto the CFGBeast executable will parse it and generate the replacement files automatically.

### Materials Replacer

![alt text](https://github.com/Outerbeast/CFGBeast/blob/main/demo_materials.png?raw=true)

This helps you create material definition files (`materials.txt`) that changes the material type for a given texture.
For more information read the [Materials Replacement Guide](https://wiki.svencoop.com/Mapping/Materials_Replacement_Guide) in the Sven Co-op Wiki.

Usage:

1. Select a GoldSrc `.wad` file using **Set WAD** — its texture names will appear in the WAD texture list.
2. Select a material kind using the radio buttons: Metal, Ventilation, Dirt, Slosh Liquid, Tile, Grate, Wood, Computer, Glass, or Flesh.
3. Click a texture in the WAD list to add it to the table with the selected kind.
4. You can also load an existing `materials.txt` using **Load**, or drag a `.txt` or `.wad` file onto the app.
5. Select a row and click **Remove** to delete it. Click **Remove** with no row selected to clear all.
6. Click **Create** to save the material definitions as a `.txt` file.

Buttons:
- `Load` - load an existing materials txt file to edit
- `Remove`- deletes a selected material in the table.
- `Create` - saves the list of materials as a text file


### Quick Create

Dragging files onto the CFGBeast executable will process them automatically without opening the GUI:

- `.cfg` files — generates CFG files for all BSPs in the current folder
- `*_motd.txt` files — generates MOTD files for all BSPs
- `.gmr` / `.gsr` files — parses and generates global replacement files

Note: `.wad` files and `.txt` material files can only be processed through the GUI's Materials tab.

### Reset Configuration

To reset the application configuration (e.g. to re-run initial setup), launch the executable with the `--reset-config` or `-r` flag:

```cmd
CFGBeast.exe --reset-config
```

```bash
./CFGBeast -r
```


# Building from source

## Prerequisites

1️⃣ Install Rust

- Visit [https://rustup.rs](https://rustup.rs) and download the Windows installer.
- Run it and accept the defaults (this installs `cargo`, `rustc`, and `rustup`).
- Close and reopen any terminal/PowerShell windows after installation.

## Build instructions

1. Download or clone the repository:

```cmd
git clone https://github.com/Outerbeast/CFGBeast.git
cd CFGBeast
```

2. Run the build script:

- Double-click build script file or run it manually from the terminal:

```cmd
build.cmd
```

```bash
./build.sh
```

The executable will be generated in the current directory.

# Cross-Platform

CFGBeast runs on both Windows and Linux. On Linux, configuration is stored in `~/.config/CFGBeast/`. Wayland sessions are automatically handled by falling back to X11.

# Feedback & Issues

If you have feedback or encounter issues, please open an issue on [GitHub Issues](https://github.com/Outerbeast/CFGBeast/issues).

# Credits

- Outerbeast - Author
- Garompa - Testing and feedback

UI powered by Slint.
