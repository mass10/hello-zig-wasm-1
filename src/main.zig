// const std = @import("std");

// pub fn main() anyerror!void {
//     // Note that info level log messages are by default printed only in Debug
//     // and ReleaseSafe build modes.
//     std.log.info("All your codebase are belong to us.", .{});
// }

// test "basic test" {
//     try std.testing.expectEqual(10, 3 + 7);
// }


// src/main.zig
const std = @import("std");

pub fn main() anyerror!void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hello, World!\n", .{});
}
