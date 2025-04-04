const std = @import("std");
const u = @import("../utils.zig");
const c = @import("../color/color.zig");
// This file cannot have any other dependencies. Might create
// circular dependencies. This should only have simple basic
// functions

pub const VERTICAL_LINE = "\u{2502}";
pub const HORIZONTAL_LINE = "\u{2500}";

pub const SQUARE_TOP_LEFT_COR = "\u{250C}";
pub const SQUARE_TOP_RIGHT_COR = "\u{2510}";
pub const SQUARE_BOT_LEFT_COR = "\u{2514}";
pub const SQUARE_BOT_RIGHT_COR = "\u{2518}";

pub const ROUND_TOP_LEFT_COR = "\u{256D}";
pub const ROUND_TOP_RIGHT_COR = "\u{256E}";
pub const ROUND_BOT_LEFT_COR = "\u{2570}";
pub const ROUND_BOT_RIGHT_COR = "\u{256F}";

pub fn draw_box(stdout: std.fs.File.Writer, rounded: bool, pos: u.Vec2, size: u.Vec2) !void {
    if (size.y < 2 or size.x < 2) {
        return error.sizeTooSmall;
    }
    const offset_x: i32 = if (pos.x > 0) @intCast(pos.x - 1) else -1;
    const offset_y: i32 = if (pos.y > 0) @intCast(pos.y - 1) else -1;
    const width: i32 = if (size.x > 2) @intCast(size.x - 2) else -1;

    try stdout.print("\x1b[H\x1b[{d}B\x1b[{d}C{s}", .{ offset_y, offset_x, if (!rounded) SQUARE_TOP_LEFT_COR else ROUND_TOP_LEFT_COR });

    for (0..size.x - 2) |_| {
        try stdout.print("{s}", .{HORIZONTAL_LINE});
    }
    try stdout.print("{s}", .{if (!rounded) SQUARE_TOP_RIGHT_COR else ROUND_TOP_RIGHT_COR});
    for (0..size.y - 2) |_| {
        try stdout.print("\x1b[G\x1b[B\x1b[{d}C{s}\x1b[{d}C{s}", .{ offset_x, VERTICAL_LINE, width, VERTICAL_LINE });
    }
    try stdout.print("\x1b[G\x1b[B\x1b[{d}C{s}", .{ offset_x, if (!rounded) SQUARE_BOT_LEFT_COR else ROUND_BOT_LEFT_COR });
    for (0..size.x - 2) |_| {
        try stdout.print("{s}", .{HORIZONTAL_LINE});
    }
    try stdout.print("{s}\n", .{if (!rounded) SQUARE_BOT_RIGHT_COR else ROUND_BOT_RIGHT_COR});
}

pub fn draw_text(stdout: std.fs.File.Writer, pos: u.Vec2, text: []const u8) !void {
    const offset_x: i32 = if (pos.x > 0) @intCast(pos.x) else -1;
    const offset_y: i32 = if (pos.y > 0) @intCast(pos.y) else -1;

    try stdout.print("\x1b[H\x1b[{d}B\x1b[{d}C{s}", .{ offset_y, offset_x, text });
}

/// A terminal cell typically has a 1x2 ratio. This function uses the unicode u2580 char
/// and the background / foreground colors to draw 2 stacked pixels in one terminal cell.
pub inline fn draw_2_stacked_pixels(stdout: std.fs.File.Writer, pos: u.Vec2, top_color: c.Color, bot_color: c.Color) !void {
    const offset_x: i32 = if (pos.x > 0) @intCast(pos.x) else -1;
    const offset_y: i32 = if (pos.y > 0) @intCast(pos.y) else -1;

    try stdout.print("\x1b[H\x1b[{d}B\x1b[{d}C\x1b[38;2;{d};{d};{d}m\x1b[48;2;{d};{d};{d}m\u{2580}\x1b[0m", .{
        offset_y,
        offset_x,
        top_color.r,
        top_color.g,
        top_color.b,
        bot_color.r,
        bot_color.g,
        bot_color.b,
    });
}

pub inline fn clear_zone(stdout: std.fs.File.Writer, pos: u.Vec2, size: u.Vec2) !void {
    const offset_x: i32 = if (pos.x > 0) @intCast(pos.x) else -1;
    const offset_y: i32 = if (pos.y > 0) @intCast(pos.y) else -1;

    try stdout.print("\x1b[H\x1b[{d}B\x1b[{d}C", .{ offset_y, offset_x });

    for (0..size.y) |_| {
        for (0..size.x) |_| {
            try stdout.print(" ", .{});
        }
        try stdout.print("\x1b[B\x1b[{d}D", .{size.x});
    }
}
