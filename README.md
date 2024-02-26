# Inkscape Figures Manager

A simple CLI tool to manage figures in created with Inkscape. It also provides shortcuts for common operations when creating technical figures that have the style of [TikZ figures](https://tikz.dev/). This crate is heavily inspired by [Gilles Castel](https://castel.dev/)'s [Inkscape figures manager](https://github.com/gillescastel/inkscape-figures) and [Inkscape shortcut manager](https://github.com/gillescastel/inkscape-shortcut-manager). The reason for creating this crate is that his tools only support Linux. This crate is written to be used in macOS (but could easily be extended to work for other operating systems).

## Installation

```bash
cargo install inkscape-figures-manager
```

Then, create a template SVG file `$HOME/.config/ifm/template.svg`. This file will be used as a template when creating new figures.

To make opening figures fast, you can use a picker tool like [choose](https://github.com/chipsenkbeil/choose) or [telescope](https://github.com/nvim-telescope/telescope.nvim) if you use NeoVim. See [my dotfiles](https://github.com/cristianpjensen/nvim-latex-config/tree/main/lua) for the extension I made for telescope (copy the `lua/telescope/` and `lua/telescope_inkscape_figures/` directories).

## Usage

TODO

### Keyboard shortcuts

TODO

## Creating Figures

For an example of the figures that can be created, see [my university notes repository](https://github.com/cristianpjensen/eth-cs-notes). Alternatively, see [Gilles Castel's master thesis](https://github.com/gillescastel/masterthesis), which contains over 100 figures.

For colors, you can use one of the qualitative color sets defined by [colorbrewer](https://colorbrewer2.org/#type=qualitative).

For advice on how to create good figures, see the [TikZ guidelines](https://tikz.dev/guidelines).

## Operating System Support

At the moment, this crate only supports macOS. It can easily be extended to support other operating systems by providing a clipboard implementation for the specific operating system (see `src/clipboard`). If this is something you are interested in, [open an issue](https://github.com/cristianpjensen/inkscape-figures-manager-rs/issues/new) or [create a pull request](https://github.com/cristianpjensen/inkscape-figures-manager-rs/compare).

Alternatively, if you want to use a crate like this one on Linux, see [Gilles Castel's Inkscape figures manager](https://github.com/gillescastel/inkscape-figures).
