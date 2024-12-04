const std = @import("std");
const ArrList = std.ArrayList([]const u8);

fn format_input(data: []const u8, alloc: std.mem.Allocator) ![][]const u8 {
    var arr = ArrList.init(alloc);
    defer arr.deinit();

    var lines = std.mem.tokenizeAny(u8, data, "\n\r");
    while (lines.next()) |line| {
        try arr.append(line);
    }

    return try arr.toOwnedSlice();
}

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const file = @embedFile("inputs/input4.txt");
    const data = try format_input(file, alloc);
    defer alloc.free(data);

    std.debug.print("Day4\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try part1(data)});
    std.debug.print("Solution Problem 2: {d}\n", .{try part2(data)});
}

fn part1(data: [][]const u8) !u32 {
    var found: u32 = 0;
    for (0..data.len) |i| {
        const calc_i: i128 = @as(i128, i);
        for (0..data[i].len) |j| {
            const calc_j: i128 = @as(i128, j);
            if (data[i][j] == 'X') {
                // horizontal
                if (j + 3 < data[i].len and data[i][j + 1] == 'M' and data[i][j + 2] == 'A' and data[i][j + 3] == 'S') {
                    found += 1;
                }
                if (calc_j - 3 >= 0 and data[i][j - 1] == 'M' and data[i][j - 2] == 'A' and data[i][j - 3] == 'S') {
                    found += 1;
                }
                // vertical
                if (i + 3 < data.len and data[i + 1][j] == 'M' and data[i + 2][j] == 'A' and data[i + 3][j] == 'S') {
                    found += 1;
                }
                if (calc_i - 3 >= 0 and data[i - 1][j] == 'M' and data[i - 2][j] == 'A' and data[i - 3][j] == 'S') {
                    found += 1;
                }
                // diagonal
                if (j + 3 < data[i].len and i + 3 < data.len and data[i + 1][j + 1] == 'M' and data[i + 2][j + 2] == 'A' and data[i + 3][j + 3] == 'S') {
                    found += 1;
                }
                if (calc_j - 3 >= 0 and i + 3 < data.len and data[i + 1][j - 1] == 'M' and data[i + 2][j - 2] == 'A' and data[i + 3][j - 3] == 'S') {
                    found += 1;
                }
                if (j + 3 < data[i].len and calc_i - 3 >= 0 and data[i - 1][j + 1] == 'M' and data[i - 2][j + 2] == 'A' and data[i - 3][j + 3] == 'S') {
                    found += 1;
                }
                if (calc_j - 3 >= 0 and calc_i - 3 >= 0 and data[i - 1][j - 1] == 'M' and data[i - 2][j - 2] == 'A' and data[i - 3][j - 3] == 'S') {
                    found += 1;
                }
            }
        }
    }
    return found;
}

fn part2(data: [][]const u8) !u32 {
    var found: u32 = 0;
    for (1..data.len - 1) |i| {
        for (1..data[i].len - 1) |j| {
            if (data[i][j] == 'A') {
                // M - S
                // - A -
                // M - S
                if (data[i - 1][j - 1] == 'M' and data[i - 1][j + 1] == 'S' and data[i + 1][j - 1] == 'M' and data[i + 1][j + 1] == 'S') {
                    found += 1;
                }
                // S - M
                // - A -
                // S - M
                if (data[i - 1][j - 1] == 'S' and data[i - 1][j + 1] == 'M' and data[i + 1][j - 1] == 'S' and data[i + 1][j + 1] == 'M') {
                    found += 1;
                }
                // M - M
                // - A -
                // S - S
                if (data[i - 1][j - 1] == 'M' and data[i - 1][j + 1] == 'M' and data[i + 1][j - 1] == 'S' and data[i + 1][j + 1] == 'S') {
                    found += 1;
                }
                // S - S
                // - A -
                // M - M
                if (data[i - 1][j - 1] == 'S' and data[i - 1][j + 1] == 'S' and data[i + 1][j - 1] == 'M' and data[i + 1][j + 1] == 'M') {
                    found += 1;
                }
            }
        }
    }
    return found;
}

const test_alloc = std.testing.allocator;

test "Example input part1" {
    const string =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;

    const data = try format_input(string, test_alloc);
    defer test_alloc.free(data);

    try std.testing.expectEqual(18, part1(data));
}

test "Example input part2" {
    const string =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;

    const data = try format_input(string, test_alloc);
    defer test_alloc.free(data);

    try std.testing.expectEqual(9, part2(data));
}
