const std = @import("std");

pub const Token = struct {
    tag: Tag,
    loc: Loc,

    pub const Loc = struct {
        start: usize,
        end: usize,
    };

    pub const keywords = std.StaticStringMap(Tag).initComptime(.{
        .{ "and", .keyword_and },
        .{ "class", .keyword_class },
        .{ "else", .keyword_else },
        .{ "false", .keyword_false },
        .{ "fun", .keyword_fun },
        .{ "for", .keyword_for },
        .{ "if", .keyword_if },
        .{ "nil", .keyword_nil },
        .{ "or", .keyword_or },
        .{ "print", .keyword_print },
        .{ "return", .keyword_return },
        .{ "super", .keyword_super },
        .{ "this", .keyword_this },
        .{ "true", .keyword_true },
        .{ "var", .keyword_var },
        .{ "while", .keyword_while },
    });

    pub fn getKeyword(bytes: []const u8) ?Tag {
        return keywords.get(bytes);
    }
    pub const Tag = enum {
        left_paren,
        right_paren,
        left_brace,
        right_brace,
        comma,
        dot,
        minus,
        plus,
        semicolon,
        slash,
        star,

        // one or two character tokens
        bang,
        bang_equal,
        equal,
        equal_equal,
        greater,
        greater_equal,
        less,
        less_equal,

        // literals
        identifier,
        string,
        number,

        // keywords
        keyword_and,
        keyword_class,
        keyword_else,
        keyword_false,
        keyword_fun,
        keyword_for,
        keyword_if,
        keyword_nil,
        keyword_or,
        keyword_print,
        keyword_return,
        keyword_super,
        keyword_this,
        keyword_true,
        keyword_var,
        keyword_while,

        eof,
        invalid,
    };
};

fn Tokenizer(comptime sentinel: anytype) type {
    return struct {
        buffer: []const u8,
        index: usize,

        pub fn init(buffer: []const u8) Tokenizer(sentinel) {
            return .{
                .buffer = buffer,
                .index = 0,
            };
        }
        pub fn next(self: *Tokenizer(sentinel)) Token {
            var result: Token = .{
                .tag = undefined,
                .loc = .{
                    .start = self.index,
                    .end = undefined,
                },
            };

            const State = enum {
                start,
                invalid,
            };

            state: switch (State.start) {
                .start => switch (self.buffer[self.index]) {
                    sentinel => {
                        if (self.index == self.buffer.len - 1) {
                            return .{
                                .tag = .eof,
                                .loc = .{
                                    .start = self.index,
                                    .end = self.index,
                                },
                            };
                        } else {
                            continue :state .invalid;
                        }
                    },
                    ' ', '\t', '\r' => {
                        self.index += 1;
                        result.loc.start = self.index;
                        continue :state .start;
                    },
                    '(' => {
                        result.tag = .left_paren;
                        self.index += 1;
                    },
                    ')' => {
                        result.tag = .right_paren;
                        self.index += 1;
                    },
                    '{' => {
                        result.tag = .left_brace;
                        self.index += 1;
                    },
                    '}' => {
                        result.tag = .right_brace;
                        self.index += 1;
                    },
                    ',' => {
                        result.tag = .comma;
                        self.index += 1;
                    },
                    '.' => {
                        result.tag = .dot;
                        self.index += 1;
                    },
                    '-' => {
                        result.tag = .minus;
                        self.index += 1;
                    },
                    '+' => {
                        result.tag = .plus;
                        self.index += 1;
                    },
                    ';' => {
                        result.tag = .semicolon;
                        self.index += 1;
                    },
                    '*' => {
                        result.tag = .slash;
                        self.index += 1;
                    },
                    else => continue :state .invalid,
                },
                .invalid => {
                    self.index += 1;
                    switch (self.buffer[self.index]) {
                        sentinel => if (self.index == self.buffer.len - 1) {
                            result.tag = .invalid;
                        } else {
                            continue :state .invalid;
                        },
                        else => continue :state .invalid,
                    }
                },
            }

            result.loc.end = self.index;
            return result;
        }
    };
}

fn runInterpreter(allocator: std.mem.Allocator, in: std.fs.File) !void {
    const reader = in.reader();

    std.debug.print("zlox interpreter v0.0.1\n", .{});
    const sentinel = '\n';

    const buf = try allocator.alloc(u8, 1 << 15);
    defer allocator.free(buf);

    while (true) {
        std.debug.print("> ", .{});
        const bytes_read = try reader.read(buf);

        var tokenizer = Tokenizer(sentinel).init(buf[0..bytes_read]);
        tokenizer: while (true) {
            const tok = tokenizer.next();
            switch (tok.tag) {
                .eof => {
                    break :tokenizer;
                },
                .invalid => {
                    std.debug.print("invalid {any}\n", .{tok});
                    break :tokenizer;
                },
                else => {
                    std.debug.print("{any}\n", .{tok});
                },
            }
        }
    }
}

pub fn main() !void {
    // var args = std.process.args();
    // _ = args.skip();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    arena.deinit();

    var gpa = std.heap.GeneralPurposeAllocator(.{ .thread_safe = true }).init;
    const allocator = gpa.allocator();
    const file = std.io.getStdIn();
    try runInterpreter(allocator, file);
}
