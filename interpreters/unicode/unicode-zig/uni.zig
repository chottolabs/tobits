const std = @import("std");

fn countUtf8(allocator: std.mem.Allocator, file: std.fs.File) !usize {
    const stat = try file.stat();
    const content = try std.zig.readSourceFileToEndAlloc(allocator, file, stat.size);

    var i: usize = 0;
    for (content) |c| {
        switch (c) {
            0b1000_0000...0b1011_1111 => {},
            else => {
                i += 1;
            },
        }
    }

    return i;
}

fn parseSourceFile(allocator: std.mem.Allocator, file: std.fs.File) !usize {
    const stat = try file.stat();
    const content = try std.zig.readSourceFileToEndAlloc(allocator, file, stat.size);

    var i: usize = 0;

    const view = std.unicode.Utf8View.initUnchecked(content);
    var it = view.iterator();

    while (std.unicode.Utf8Iterator.nextCodepointSlice(&it)) |_| {
        i += 1;
    }
    return i;
}

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();

    var gpa = std.heap.GeneralPurposeAllocator(.{ .thread_safe = true }).init;

    const allocator = gpa.allocator();

    if (args.next()) |f| {
        const file = try std.fs.cwd().openFile(f, .{});
        _ = try parseSourceFile(allocator, file);
        // _ = try countUtf8(allocator, file);
        // std.debug.print("{d}\n", .{i});
    }
}
