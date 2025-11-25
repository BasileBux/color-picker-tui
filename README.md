# TUI color picker

This is a terminal based color picker. It is a little tool to help you choose
pick colors and translate them from / to different formats (HEX, RGB, HSV). It
is highly inspired by [this website](https://htmlcolorcodes.com/).

## Installation

I don't do packaging or releases. If you want to use it, you will need to build
it from source. Just install install rust stuff and run `cargo build --release`
and you will get your binary.

The TUI draws a big square for color selection. This square could not appear
completely square depending on your font. For better looking squares, I
recommend using a font which has a 1:2 ratio such as
[FiraCode](https://github.com/tonsky/FiraCode).

## Usage

Just drag your mouse over the gradients to select a color. You can also give the
colors in the input fields on the right. To validate the input, just press `enter`.
You can also paste in the input fields with `Ctrl + Shift + V` or `p`. To copy the
color, click on the relevant format at the top. 

To quit the program, you can do `Ctrl + C`, `escape` or `q`. 

> [!WARNING]
> Clipboard support is only available on wayland with `wl-clipboard` installed.
> Better implementation coming soon.

## Note

I needed to learn some Rust basics fast so I remade [this](https://github.com/BasileBux/color-picker-tui-zig)
project I once wrote in Zig. It looks the same but is now written in Rust so it's
blazingly fast™. And the codebase is way cleaner. Also, this time I read about color
representations so the saturation and value picker is actually correct.
