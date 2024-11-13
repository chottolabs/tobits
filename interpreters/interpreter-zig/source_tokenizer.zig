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

const SourceTokenizer = struct {
    buffer: [:0]const u8,
    index: usize,

    pub fn init(buffer: [:0]const u8) SourceTokenizer {
        return .{
            .buffer = buffer,
            .index = 0,
        };
    }
    pub fn next(self: *SourceTokenizer) ?Token {
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
            bang,
            equal,
            angle_bracket_left,
            angle_bracket_right,
            int,
            int_period,
            float,
            identifier,
        };

        state: switch (State.start) {
            .start => switch (self.buffer[self.index]) {
                0 => {
                    if (self.index == self.buffer.len) {
                        return null;
                    } else {
                        continue :state .invalid;
                    }
                },
                ' ', '\n', '\t', '\r' => {
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
                '!' => continue :state .bang,
                '=' => continue :state .equal,
                '<' => continue :state .angle_bracket_left,
                '>' => continue :state .angle_bracket_right,
                '0'...'9' => {
                    result.tag = .number;
                    self.index += 1;
                    continue :state .int;
                },
                'a'...'z', 'A'...'Z', '_' => {
                    result.tag = .identifier;
                    continue :state .identifier;
                },
                else => continue :state .invalid,
            },
            .bang => {
                self.index += 1;
                switch (self.buffer[self.index]) {
                    '=' => {
                        result.tag = .bang_equal;
                        self.index += 1;
                    },
                    else => result.tag = .bang,
                }
            },
            .equal => {
                self.index += 1;
                switch (self.buffer[self.index]) {
                    '=' => {
                        result.tag = .equal_equal;
                        self.index += 1;
                    },
                    else => result.tag = .equal,
                }
            },
            .angle_bracket_left => {
                self.index += 1;
                switch (self.buffer[self.index]) {
                    '=' => {
                        result.tag = .less_equal;
                        self.index += 1;
                    },
                    else => result.tag = .less,
                }
            },
            .angle_bracket_right => {
                self.index += 1;
                switch (self.buffer[self.index]) {
                    '=' => {
                        result.tag = .greater_equal;
                        self.index += 1;
                    },
                    else => result.tag = .greater,
                }
            },
            .int => switch (self.buffer[self.index]) {
                '.' => continue :state .int_period,
                '_', 'a'...'d', 'f'...'o', 'q'...'z', 'A'...'D', 'F'...'O', 'Q'...'Z', '0'...'9' => {
                    self.index += 1;
                    continue :state .int;
                },
                // 'e', 'E', 'p', 'P' => {
                //     continue :state .int_exponent;
                // },
                else => {},
            },
            .int_period => {
                self.index += 1;
                switch (self.buffer[self.index]) {
                    '_', 'a'...'d', 'f'...'o', 'q'...'z', 'A'...'D', 'F'...'O', 'Q'...'Z', '0'...'9' => {
                        self.index += 1;
                        continue :state .float;
                    },
                    // 'e', 'E', 'p', 'P' => {
                    //     continue :state .float_exponent;
                    // },
                    else => self.index -= 1,
                }
            },
            .float => switch (self.buffer[self.index]) {
                '_', 'a'...'d', 'f'...'o', 'q'...'z', 'A'...'D', 'F'...'O', 'Q'...'Z', '0'...'9' => {
                    self.index += 1;
                    continue :state .float;
                },
                // 'e', 'E', 'p', 'P' => {
                //     continue :state .float_exponent;
                // },
                else => {},
            },
            .identifier => {
                self.index += 1;
                switch (self.buffer[self.index]) {
                    'a'...'z', 'A'...'Z', '_', '0'...'9' => continue :state .identifier,
                    else => {
                        const ident = self.buffer[result.loc.start..self.index];
                        if (Token.getKeyword(ident)) |tag| {
                            result.tag = tag;
                        }
                    },
                }
            },
            .invalid => {
                self.index += 1;
                switch (self.buffer[self.index]) {
                    0 => if (self.index == self.buffer.len) {
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

fn parseSourceFile(allocator: std.mem.Allocator, in: std.fs.File) !void {
    const out = std.io.getStdOut();
    var buf = std.io.bufferedWriter(out.writer());
    var w = buf.writer();

    const stat = try in.stat();
    const content = try std.zig.readSourceFileToEndAlloc(allocator, in, stat.size);

    var tokenizer = SourceTokenizer.init(content);
    while (tokenizer.next()) |tok| {
        switch (tok.tag) {
            .eof, .invalid => {
                break;
            },
            else => {
                // std.debug.print("{s}\n", .{@tagName(tok.tag)});
                try w.print("{s}\n", .{@tagName(tok.tag)});
                try buf.flush();
            },
        }
    }
    // std.debug.print("\n{any}\n", .{content});
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
