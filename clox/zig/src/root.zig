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
    writer: std.io.AnyWriter,

    pub fn disassemble_chunk(self: *Disassembler, chunk: ChunkArray, name: []const u8) !void {
        debug.disassemble_chunk(chunk, name, self.writer);
    }

    pub fn disassemble_instruction(self: *Disassembler, chunk: Chunk, offset: usize) !usize {
        debug.disassemble_instruction(chunk, offset, self.writer);
    }

    pub fn constant_instruction(self: *Disassembler, name: []const u8, chunk: Chunk, offset: usize) !usize {
        debug.constant_instruction(name, chunk, offset, self.writer);
    }

    pub fn print_value(self: *Disassembler, value: Value) !void {
        debug.print_value(value, self.writer);
    }

    pub fn simple_instruction(self: *Disassembler, name: []const u8, offset: usize) !usize {
        debug.simple_instruction(name, offset, self.writer);
    }
};
