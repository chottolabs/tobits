const std = @import("std");

fn runInterpreter(in: std.fs.File) !void {
    const reader = in.reader();
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    arena.deinit();

    const allocator = arena.allocator();

    std.debug.print("zlox interpreter v0.0.1\n", .{});
    std.debug.print("> ", .{});
    while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', 1 << 30)) |line| {
        std.debug.print("{s}\n", .{line});
        std.debug.print("> ", .{});
    }
}

fn parseSourceFile(in: std.fs.File) !void {
    const reader = in.reader();
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    arena.deinit();

    const allocator = arena.allocator();

    const content = try reader.readAllAlloc(allocator, 1 << 30);
    std.debug.print("{s}", .{content});
}

pub fn main() !void {
    var args = std.process.args();
    _ = args.skip();

    if (args.next()) |f| {
        const file = try std.fs.openFileAbsolute(f, .{});
        try parseSourceFile(file);
    } else {
        const file = std.io.getStdIn();
        try runInterpreter(file);
    }
}
