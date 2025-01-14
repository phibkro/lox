const std = @import("std");
const testing = std.testing;
const debug = @import("debug.zig");

pub const OpCode = enum(u8) {
    CONSTANT,
    RETURN,
};

pub const Value = f64;
pub const ValueArray = std.ArrayList(Value);

pub const Chunk = struct {
    code: u8,
    constants: ?std.ArrayList(Value),
};
pub const ChunkArray = std.ArrayList(Chunk);

pub const Disassembler = struct {
    const Self = @This();

    writer: std.io.AnyWriter,

    pub fn disassemble_chunk(self: *Self, chunk: ChunkArray, name: []const u8) !void {
        try debug.disassemble_chunk(chunk, name, self.writer);
    }

    pub fn disassemble_instruction(self: *Self, chunk: Chunk, offset: usize) !usize {
        try debug.disassemble_instruction(chunk, offset, self.writer);
    }

    pub fn constant_instruction(self: *Self, name: []const u8, chunk: Chunk, offset: usize) !usize {
        try debug.constant_instruction(name, chunk, offset, self.writer);
    }

    pub fn print_value(self: *Self, value: Value) !void {
        try debug.print_value(value, self.writer);
    }

    pub fn simple_instruction(self: *Self, name: []const u8, offset: usize) !usize {
        try debug.simple_instruction(name, offset, self.writer);
    }
};
