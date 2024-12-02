//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.
const std = @import("std");

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile("inputs/input1.txt", .{});
    defer file.close();
    const file_size = (try file.stat()).size;
    const file_contents = try file.reader().readAllAlloc(allocator, file_size);
    defer allocator.free(file_contents);

    std.debug.print("Day1\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try problem1(file_contents, 1000)});
    std.debug.print("Solution Problem 2: {d}\n", .{try problem2(file_contents, 1000)});
}

fn problem1(data: []const u8, comptime array_size: usize) !u32 {
    const trimmed = std.mem.trim(u8, data, "\n\r ");
    var lines = std.mem.tokenizeAny(u8, trimmed, "\n");

    var array1: [array_size]i32 = undefined;
    var array2: [array_size]i32 = undefined;

    var i: usize = 0;
    while (lines.next()) |x| {
        var values = std.mem.splitSequence(u8, x, "   ");
        const one = values.first();
        const two = values.rest();

        array1[i] = try std.fmt.parseInt(i32, one, 10);
        array2[i] = try std.fmt.parseInt(i32, two, 10);

        i += 1;
    }

    std.mem.sort(i32, &array1, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, &array2, {}, comptime std.sort.asc(i32));

    var result: u32 = 0;

    for (0..array_size) |y| {
        const pri = @abs(array1[y] - array2[y]);
        result += pri;
    }

    return result;
}

fn problem2(data: []const u8, comptime array_size: usize) !i32 {
    const trimmed = std.mem.trim(u8, data, "\n\r ");
    var lines = std.mem.splitScalar(u8, trimmed, '\n');
    var array1: [array_size]i32 = undefined;
    var array2: [array_size]i32 = undefined;

    var i: usize = 0;
    while (lines.next()) |x| {
        var values = std.mem.splitSequence(u8, x, "   ");
        const one = values.first();
        const two = values.rest();

        array1[i] = try std.fmt.parseInt(i32, one, 10);
        array2[i] = try std.fmt.parseInt(i32, two, 10);

        i += 1;
    }

    var result: i32 = 0;
    for (array1) |x| {
        result += x * count(&array2, x);
    }
    return result;
}

fn count(array: []i32, key: i32) i32 {
    var num: i32 = 0;
    for (array) |value| {
        if (value == key) {
            num += 1;
        }
    }
    return num;
}

test "example input test problem1" {
    const test_string =
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    ;
    try std.testing.expectEqual(11, problem1(test_string, 6));
}

test "example input test problem2" {
    const test_string =
        \\3   4
        \\4   3
        \\2   5
        \\1   3
        \\3   9
        \\3   3
    ;
    try std.testing.expectEqual(31, problem1(test_string, 6));
}
