const std = @import("std");
const term = @import("term.zig");
const shade_pick = @import("color/shade-picker.zig");
const hue_pick = @import("color/hue-picker.zig");
const commons = @import("commons.zig");
const color_input = @import("color/input.zig");

const MIN_WIDTH: u32 = @intFromFloat(commons.SIZE_GLOBAL + 2 * commons.SPACING + hue_pick.WIDTH + 2 * commons.SPACING + color_input.WIDTH);
const MIN_HEIGHT: u32 = @intFromFloat(commons.SIZE_GLOBAL / 2 + 5);

var window_resized = std.atomic.Value(bool).init(false);
fn handleSigwinch(sig: c_int) callconv(.C) void {
    _ = sig;
    window_resized.store(true, .seq_cst);
}

var sigint_received: std.atomic.Value(bool) = std.atomic.Value(bool).init(false);
fn handleSigint(_: c_int) callconv(.C) void {
    sigint_received.store(true, .seq_cst);
}

pub const Ui = struct {
    ctx: *term.TermContext,
    exit_sig: bool,

    shade_picker: shade_pick.ShadePicker,
    hue_picker: hue_pick.HuePicker,
    input: color_input.ColorInput,

    win_too_small: bool,

    pub fn init(ctx: *term.TermContext, allocator: std.mem.Allocator) !Ui {
        // Signal handling
        const sigint_act = std.os.linux.Sigaction{
            .handler = .{ .handler = handleSigint },
            .mask = std.os.linux.empty_sigset,
            .flags = 0,
        };
        _ = std.os.linux.sigaction(std.os.linux.SIG.INT, &sigint_act, null);
        const sigwinch_act = std.os.linux.Sigaction{
            .handler = .{ .handler = handleSigwinch },
            .mask = std.os.linux.empty_sigset,
            .flags = 0,
        };
        _ = std.os.linux.sigaction(std.os.linux.SIG.WINCH, &sigwinch_act, null);

        return Ui{
            .ctx = ctx,
            .exit_sig = false,
            .shade_picker = try shade_pick.ShadePicker.init(ctx.stdout, allocator, .{ .x = 1, .y = 2 }),
            .hue_picker = hue_pick.HuePicker.init(ctx.stdout, .{ .x = commons.SIZE_GLOBAL + commons.SPACING, .y = 2 }),
            // .hue_picker = hue_pick.HuePicker.init(ctx.stdout, .{ .x = commons.SIZE_GLOBAL + commons.SPACING + 30, .y = 2 }),
            .input = try color_input.ColorInput.init(ctx.stdout, .{ .x = commons.SIZE_GLOBAL + 2 * commons.SPACING + hue_pick.WIDTH - 1, .y = 2 }, allocator),
            .win_too_small = false,
        };
    }

    pub fn deinit(self: *Ui) void {
        self.input.deinit();
    }

    pub fn run(self: *Ui) !void {
        self.hue_picker.render();
        const pid = std.os.linux.getpid();
        _ = std.os.linux.kill(pid, std.os.linux.SIG.WINCH);
        while (!self.exit_sig) {
            try self.signal_manager();
            const in: term.Input = self.ctx.getInput() catch break;
            try self.shade_picker.update(in);
            self.hue_picker.update(in);
            const input_color = try self.input.update(in);
            switch (in) {
                term.InputType.control => |control| {
                    const unwrapped_control = control orelse term.ControlKeys.None;
                    switch (unwrapped_control) {
                        term.ControlKeys.Escape => {
                            self.exit_sig = true;
                        },
                        else => {
                            // Handle other control keys
                        },
                    }
                },
                term.InputType.utf8 => |_| {},
                term.InputType.mouse => |_| {},
            }
            if (self.win_too_small) continue;

            if (input_color) |col| {
                self.shade_picker.selected_color = col;
                self.shade_picker.select_update = true;
                self.input.color = col;
                self.input.updateColor(col);
            }

            if (self.hue_picker.select_update) {
                self.hue_picker.select_update = false;
                self.shade_picker.color = self.hue_picker.selected_hue;
                self.shade_picker.render_update = true;
                self.input.updateColor(self.shade_picker.color);
            }

            if (self.shade_picker.select_update) {
                if (self.ctx.background_color) |bg| {
                    try self.ctx.stdout.print("\x1B[48;2;{};{};{}m", .{ bg[0], bg[1], bg[2] });
                }

                try self.ctx.stdout.print("\x1b[H", .{}); // Move cursor to top left

                try self.ctx.stdout.print("\x1b[K", .{});
                try self.ctx.stdout.print("\x1b[1C", .{});
                try self.ctx.stdout.print("\x1B[0m\x1B[48;2;{};{};{}m{s}{s}\x1B[0m", .{
                    self.shade_picker.selected_color.r,
                    self.shade_picker.selected_color.g,
                    self.shade_picker.selected_color.b,
                    commons.WHITESPACE,
                    commons.WHITESPACE,
                });
                const hex_color = self.shade_picker.selected_color.toHex();
                const hsl_color = self.shade_picker.selected_color.toHsl();
                try self.ctx.stdout.print("{s}HEX: #{x:0>6}{s}RGB: {d}, {d}, {d}{s}HSL: {d:.0}, {d:.2}%, {d:.2}%", .{
                    commons.WHITESPACE,
                    hex_color,
                    commons.WHITESPACE,
                    self.shade_picker.selected_color.r,
                    self.shade_picker.selected_color.g,
                    self.shade_picker.selected_color.b,
                    commons.WHITESPACE,
                    hsl_color.h,
                    hsl_color.s * 100,
                    hsl_color.l * 100,
                });
                self.input.updateColor(self.shade_picker.selected_color);
                self.shade_picker.select_update = false;
            }

            self.shade_picker.calculateTableAndRender();
            try self.input.render();
        }
    }

    fn signal_manager(self: *Ui) !void {
        if (sigint_received.load(.seq_cst)) {
            sigint_received.store(false, .seq_cst);
            self.exit_sig = true;
            return;
        }
        if (window_resized.load(.seq_cst)) {
            window_resized.store(false, .seq_cst);
            try self.ctx.getTermSize();
            try self.ctx.stdout.print("\x1b[2J\x1b[H", .{}); // Clear screen and move cursor to top left
            if (self.ctx.win_size.cols < MIN_WIDTH or self.ctx.win_size.rows < MIN_HEIGHT) {
                self.win_too_small = true;
                try self.ctx.stdout.print("Window too small", .{});
                try self.ctx.stdout.print("Min size: {d} x {d}", .{ MIN_WIDTH, MIN_HEIGHT });
            } else {
                self.win_too_small = false;
            }
            self.shade_picker.render_update = true;
            self.hue_picker.render();
        }
    }
};
