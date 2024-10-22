const std = @import("std");

fn parseSourceFile(allocator: std.mem.Allocator, file: std.fs.File) !void {
    const stat = try file.stat();
    const content = try std.zig.readSourceFileToEndAlloc(allocator, file, stat.size);

    var i: usize = 0;
    // for (content) |c| {
    //     switch (c) {
    //         0b1000_0000...0b1011_1111 => {},
    //         else => {
    //             i += 1;
    //         },
    //     }
    // }

    const view = std.unicode.Utf8View.initUnchecked(content);
    var it = view.iterator();

    // std.debug.print("{d}", .{content.len});
    // while (std.unicode.Utf8Iterator.nextCodepoint(&it)) |_| {
    //     i += 1;
    // }

    while (std.unicode.Utf8Iterator.nextCodepointSlice(&it)) |_| {
        i += 1;
    }
    std.debug.print("{d}\n", .{i});
}

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();

    var gpa = std.heap.GeneralPurposeAllocator(.{ .thread_safe = true }).init;

    const allocator = gpa.allocator();

    if (args.next()) |f| {
        const file = try std.fs.cwd().openFile(f, .{});
        try parseSourceFile(allocator, file);
    }
}
