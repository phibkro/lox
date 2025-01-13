const std = @import("std");
const root = @import("root.zig");

pub fn disassemble_chunk(chunk: root.ChunkArray, name: []const u8, writer: std.io.AnyWriter) !void {
    try writer.print("== {s} ==\n", .{name});

    for (chunk.items, 0..) |value, i| {
        _ = try disassemble_instruction(value, i, writer);
    }
}

pub fn disassemble_instruction(chunk: root.Chunk, offset: usize, writer: std.io.AnyWriter) !usize {
    try writer.print("{d} ", .{offset});

    const instruction = chunk.code;

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
    const constant = chunk.code;

    try writer.print("{s} {d} '", .{ name, constant });
    try print_value(chunk.constants.?.items[constant], writer);
    try writer.print("'\n", .{});
    return offset + 2;
}

pub fn print_value(value: root.Value, writer: std.io.AnyWriter) !void {
    try writer.print("{d}", .{value});
}

pub fn simple_instruction(name: []const u8, offset: usize, writer: std.io.AnyWriter) !usize {
    try writer.print("{s}\n", .{name});
    return offset + 1;
}
