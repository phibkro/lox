const std = @import("std");
const root = @import("root.zig");

pub fn disassemble_chunk(chunk: root.Chunk, name: []const u8, writer: std.io.AnyWriter) !void {
    try writer.print("== {s} ==\n", .{name});

    var offset: usize = 0;
    while (offset < chunk.code.items.len) {
        offset = try disassemble_instruction(chunk, offset, writer);
    }
}

pub fn disassemble_instruction(chunk: root.Chunk, offset: usize, writer: std.io.AnyWriter) !usize {
    try writer.print("{d} ", .{offset});

    const instruction = chunk.code.items[offset];

    switch (instruction) {
        @intFromEnum(root.OpCode.CONSTANT) => return try constant_instruction("OP_CONSTANT", chunk, offset, writer),
        @intFromEnum(root.OpCode.RETURN) => return try simple_instruction("OP_RETURN", offset, writer),
        else => {
            try writer.print("Unknown opcode {d}\n", .{instruction});
            return offset + 1;
        },
    }
}

pub fn constant_instruction(name: []const u8, chunk: root.Chunk, offset: usize, writer: std.io.AnyWriter) !usize {
    const constant = chunk.code.items[offset + 1];

    try writer.print("{s} {d} '", .{ name, constant });
    try print_value(chunk.constants.items[constant], writer);
    try writer.print("'\n", .{});
    return offset + 2;
}

fn print_value(value: root.Value, writer: std.io.AnyWriter) !void {
    try writer.print("{d}", .{value});
}

pub fn simple_instruction(name: []const u8, offset: usize, writer: std.io.AnyWriter) !usize {
    try writer.print("{s}\n", .{name});
    return offset + 1;
}
