const std = @import("std");
const cookie = @import("cookie.zig").cookie;

pub fn main() !void {
    var args = std.process.args();
    _ = args.next();
    const day = args.next() orelse {
        std.debug.print("No day given to get the inputs for!\n", .{});
        return;
    };

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var client = std.http.Client{ .allocator = allocator };
    defer client.deinit();

    const url = try std.fmt.allocPrint(allocator, "https://adventofcode.com/2024/day/{s}/input", .{day});
    defer allocator.free(url);

    const headers = [_]std.http.Header{.{
        .name = "Cookie",
        .value = cookie,
    }};
    var buf: [4096]u8 = undefined;

    const uri = try std.Uri.parse(url);
    var req = try client.open(.GET, uri, .{
        .server_header_buffer = &buf,
        .extra_headers = &headers,
    });
    defer req.deinit();

    try req.send();
    try req.finish();
    try req.wait();

    if (req.response.status != std.http.Status.ok) {
        std.debug.print("{any}", .{req.response.status});
        return error.WrongStatusResponse;
    }

    var reader = req.reader();
    const body = try reader.readAllAlloc(allocator, 1024 * 1024 * 4);
    defer allocator.free(body);

    const file_name = try std.fmt.allocPrint(allocator, "src/inputs/input{s}.txt", .{day});
    defer allocator.free(file_name);

    const file = try std.fs.cwd().createFile(file_name, .{ .read = true });
    defer file.close();

    try file.writeAll(body);

    std.debug.print("Created file {s}\n", .{file_name});
}
