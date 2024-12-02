const std = @import("std");

pub fn main() !void {
    // Create a general purpose allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create a HTTP client
    var client = std.http.Client{ .allocator = allocator };
    defer client.deinit();

    const day = std.os.argv[1];
    const url = try std.fmt.allocPrint(allocator, "https://adventofcode.com/2024/day/{s}/input", .{day});
    defer allocator.free(url);

    var buf: [4096]u8 = undefined;

    const uri = try std.Uri.parse(url);
    var req = try client.open(.GET, uri, .{ .server_header_buffer = &buf });
    defer req.deinit();

    try req.send();
    try req.finish();
    try req.wait();
    if (req.response.status != std.http.Status.ok) {
        std.debug.print("{any}", .{req.response.status});
        return error.WrongStatusResponse;
    }

    std.debug.print("Worked", .{});
}
