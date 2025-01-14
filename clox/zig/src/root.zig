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
    const Self = @This();
    code: std.ArrayList(u8),
    constants: std.ArrayList(Value),

    pub fn init(allocator: std.mem.Allocator) !Chunk {
        return .{
            .code = std.ArrayList(u8).init(allocator),
            .constants = std.ArrayList(Value).init(allocator),
        };
    }

    pub fn deinit(self: *Self) !void {
        self.code.deinit();
        self.constants.deinit();
    }

    pub fn write(self: *Self, byte: u8) !void {
        try self.code.append(byte);
    }

    /// Returns the index of the added constant
    pub fn add_constant(self: *Self, value: Value) !u8 {
        try self.constants.append(value);
        return @intCast(self.constants.items.len - 1);
    }
};

pub const Disassembler = struct {
    const Self = @This();

    writer: std.io.AnyWriter,

    pub fn disassemble_chunk(self: *Self, chunk: Chunk, name: []const u8) !void {
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
