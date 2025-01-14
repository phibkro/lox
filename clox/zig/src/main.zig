const std = @import("std");
const root = @import("root.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const std_out = std.io.getStdOut().writer().any();
    var disassembler = root.Disassembler{ .writer = std_out };

    var chunk_array = root.ChunkArray.init(allocator);
    var constant_array = root.ValueArray.init(allocator);
    try constant_array.append(1.2);

    try chunk_array.append(.{
        .code = @intFromEnum(root.OpCode.CONSTANT),
        .constants = constant_array,
    });

    const op_code = @intFromEnum(root.OpCode.RETURN);
    try chunk_array.append(.{
        .code = op_code,
        .constants = null,
    });
    try disassembler.disassemble_chunk(chunk_array, "test chunk");
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
