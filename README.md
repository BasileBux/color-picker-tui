# TUI color picker

This is a terminal based color picker. It is a little tool to help you choose
pick colors and translate them from / to different formats (HEX, RGB, HSV). It
is highly inspired by [this website](https://htmlcolorcodes.com/).

<https://github.com/user-attachments/assets/8fed3262-f9c1-4451-bd7c-3993a934c965>

## Usage

Just drag your mouse over the gradients to select a color. You can also give the
colors in the input fields on the right. To validate the input, just press `enter`.
You can also paste in the input fields with `Ctrl + Shift + V` or `p`. To copy the
color, click on the relevant format at the top. 

To quit the program, you can do `Ctrl + C`, `escape` or `q`. 

### Keybindings

With `y`, you can copy the color and you will be prompted to choose the format by
pressing the relevant key: `x` for HEX, `r` for RGB and `h` for HSV (you can see
that at the end of the demo video).

With `p`, you can paste a color from your clipboard into the input fields. The
color format will be automatically detected.

> [!WARNING]
> Clipboard features are a work in progress. For now, only wayland is supported
> through `wl-clipboard`. Also, only HEX format is supported for pasting.

## Installation

I don't do packaging or releases. If you want to use it, you will need to build
it from source. Just [install rust stuff](https://rust-lang.org/tools/install/)
and run `cargo build --release` and you will get your binary.

The TUI draws a big square for color selection. This square could not appear
completely square depending on your font. For better looking squares, I
recommend using a font which has a 1:2 ratio such as
[FiraCode](https://github.com/tonsky/FiraCode).

## Note

I needed to learn some Rust basics fast so I remade [this](https://github.com/BasileBux/color-picker-tui-zig)
project I once wrote in Zig. It looks the same but is now written in Rust so it's
blazingly fast™. And the codebase is way cleaner. Also, this time I read about color
representations so the saturation and value picker is actually correct.
