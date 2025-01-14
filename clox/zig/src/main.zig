const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const std_out = std.io.getStdOut().writer().any();
    var disassembler = root.Disassembler{ .writer = std_out };

    var chunk = try root.Chunk.init(allocator);

    const constant = try chunk.add_constant(1.2);
    try chunk.write(@intFromEnum(root.OpCode.CONSTANT), 123);
    try chunk.write(constant, 123);

    try chunk.write(@intFromEnum(root.OpCode.RETURN), 123);

    try disassembler.disassemble_chunk(chunk, "test chunk");
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
