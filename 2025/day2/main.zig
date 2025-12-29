const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const content = try std.fs.cwd().readFileAlloc(allocator, "input.txt", 1024 * 1024);
    defer allocator.free(content); // Free memory when done

    var iterator = std.mem.splitScalar(u8, content, '\n');

    var ids = std.ArrayList([2]i64){};
    defer ids.deinit(allocator);

    const line = iterator.next();
    var ranges = std.mem.splitScalar(u8, line.?, ',');
    while (ranges.next()) |range| {
        var parts = std.mem.splitScalar(u8, range, '-');

        const first_str = parts.next() orelse continue;
        const second_str = parts.next() orelse continue;

        const start = try std.fmt.parseInt(i64, std.mem.trim(u8, first_str, " \r"), 10);
        const end = try std.fmt.parseInt(i64, std.mem.trim(u8, second_str, " \r"), 10);

        try ids.append(allocator, .{ start, end });
    }
    for (ids.items, 0..) |pair, i| {
        std.debug.print("Row {d}: ID1={d}, ID2={d}\n", .{ i, pair[0], pair[1] });
    }
}
