const std = @import("std");

const Tag = enum {
    left_paren, right_paren, left_brace, right_brace,
    comma, dot, minus, plus, semicolon, slash, star,

    // one or two character tokens
    bang, bang_equal, equal, equal_equal,
    greater, greater_equal, less, less_equal,

    // literals
    literal_identifier, literal_string, literal_number,

    // keywords
    keyword_and, keyword_class, keyword_else, keyword_false, keyword_fun, keyword_for,
    keyword_if, keyword_nil, keyword_or, keyword_print, keyword_return, keyword_super,
    keyword_this, keyword_true, keyword_var, keyword_while,

    eof,
};

pub const keywords = blk: {
    @setEvalBranchQuota(1500);
    var m: [@typeInfo(Tag).@"enum".fields.len]struct {[]const u8, Tag} = undefined;

    for (@typeInfo(Tag).@"enum".fields, 0..) |f, i| {
        // var buf: [f.name.len]u8 = undefined;
        // _ = std.ascii.lowerString(&buf, f.name[0..]);
        // const new_field = buf;
        // m[i] = .{&new_field, @field(Tag, f.name)};
        m[i] = .{f.name, @field(Tag, f.name)};
    }
    break :blk std.StaticStringMap(Tag).initComptime(m);
};

pub fn getKeyword(bytes: []const u8) ?Tag {
    return keywords.get(bytes);
}

pub fn main() !void {
    // inline for (@typeInfo(Tag).@"enum".fields) |f| {
    //     std.debug.print("{s}\n", .{f.name});
    // }

    // std.debug.print("{any}\n", .{getKeyword("bang")});
    std.debug.print("{any}\n", .{keywords});
}
