# Inkscape Figures Manager

A simple CLI tool to manage figures in created with Inkscape. It also provides shortcuts for common
operations when creating technical figures that have the style of [TikZ figures](https://tikz.dev/).
This crate is heavily inspired by [Gilles Castel](https://castel.dev/)'s
[Inkscape figures manager](https://github.com/gillescastel/inkscape-figures) and
[Inkscape shortcut manager](https://github.com/gillescastel/inkscape-shortcut-manager). The reason
for creating this crate is that his tools only support Linux. This crate is written to be used in
macOS (but could easily be extended to work for other operating systems).

## Installation

```bash
cargo install inkscape-figures-manager
```

Then, create a template SVG file `$HOME/.config/ifm/template.svg`. This file will be used as a
template when creating new figures. I recommend pinning commonly used colors and setting the
dimensions. The dimensions should be set to be equal to the dimensions of your LaTeX document, which
can be set using the geometry package. An example template is `template.svg` in this repository.

## Usage

```
inkscape-figures-manager start
inkscape-figures-manager list
inkscape-figures-manager new <path>
inkscape-figures-manager edit <path>
```

`list` lists all SVG files in all subdirectories from where the command is run. This can be used by a
picker to select a figure to edit. I recommend using a picker to quickly open figures from within a
project. You can use a picker tool such as [choose](https://github.com/chipsenkbeil/choose). I use
[telescope.nvim](https://github.com/nvim-telescope/telescope.nvim) in my Neovim setup; see
[my config](https://github.com/cristianpjensen/nvim-latex-config/tree/main/lua) for the
extension I made for telescope (copy the `lua/telescope/` and `lua/telescope_inkscape_figures/`
directories).

`new` creates a new figure with the specified path, and opens it. This command requires a template
SVG file in `$HOME/.config/ifm/template.svg`, so make sure to create one. In a Neovim setup, you can
use the following keymap to create new figures with Ctrl+F,
```
vim.keymap.set(
    "i",
    "<C-f>",
    "<Esc><cmd>exec 'r!inkscape-figures-manager new -f -d figures -l \"'.getline('.').'\"'<CR>kkkkkkddjjjf{a"
)
```

`edit` opens the inkscape for a specified path. This command will error if the path does not exist,
or if the file is not an SVG.

### File watcher

You can start the file watcher and the shortcut listener by executing the following command,

```bash
inkscape-figures-manager start
```

This command does two things:
 1. Whenever an SVG is saved in any subdirectory where the command was executed, it will compile it
    as a PDF to be used in LaTeX;
 2. It listens for keyboard shortcuts, as defined in the next section.

### Keyboard shortcuts

To apply a style to an Inkscape object, select the object and hold Alt/Option. While holding Alt,
press the keys as defined below. It will remove all the current styles of the object and put the
style as specified.

|                   | Style (KEY) |            |                |
|-------------------|-------------|------------|----------------|
| **Stroke width:** | Normal (1)  | Thick (2)  | Very thick (3) |
| **Stroke:**       | Solid (Q)   | Dashed (W) | Dotted (E)     |
| **Fill**          | White (A)   | Gray (S)   | Black (D)      |
| **Arrow**         | Start (Z)   | End (X)    | --             |

> [!IMPORTANT]
> On macOS, the shortcuts require you to give accessibility permissions to the terminal that you are
> running the command in.

> [!WARNING]
> The program grabs all keyboard interactions while Alt/Option is held. So, no other shortcuts
> involving Alt/Option will be triggered while it is running.

> [!NOTE]
> The defined keyboard shortcuts have no semantic meaning, but are defined to be easily reachable
> with only the left hand on a US keyboard. 

## Creating Figures

For an example of the figures that can be created, see
[my university notes repository](https://github.com/cristianpjensen/eth-cs-notes). Alternatively, see
[Gilles Castel's master thesis](https://github.com/gillescastel/masterthesis), which contains over
100 figures.

For colors, you can use one of the qualitative color sets defined by
[colorbrewer](https://colorbrewer2.org/#type=qualitative).

For advice on how to create good figures, see the [TikZ guidelines](https://tikz.dev/guidelines).

## Operating System Support

At the moment, this crate only supports macOS. It can easily be extended to support other operating
systems by providing a clipboard implementation for the specific operating system (see
`src/clipboard`). If this is something you are interested in,
[open an issue](https://github.com/cristianpjensen/inkscape-figures-manager-rs/issues/new) or
[create a pull request](https://github.com/cristianpjensen/inkscape-figures-manager-rs/compare).

Alternatively, if you want to use a crate like this one on Linux, see
[Gilles Castel's Inkscape figures manager](https://github.com/gillescastel/inkscape-figures).
