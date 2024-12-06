const std = @import("std");
const ArrList = std.ArrayList(u8);
const Grid = std.ArrayList([]u8);

const direction = enum {
    up,
    down,
    left,
    right,
};

fn format_input(data: []const u8, alloc: std.mem.Allocator) ![][]u8 {
    var grid = Grid.init(alloc);
    defer grid.deinit();

    var lines = std.mem.tokenizeAny(u8, data, "\n\r");
    while (lines.next()) |line| {
        var arr = ArrList.init(alloc);

        for (line) |x| {
            try arr.append(x);
        }
        try grid.append(try arr.toOwnedSlice());
    }

    return try grid.toOwnedSlice();
}

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const file = @embedFile("inputs/input6.txt");
    const data = try format_input(file, alloc);
    defer {
        for (data) |da| {
            defer alloc.free(da);
        }
        alloc.free(data);
    }
    std.debug.print("Day6\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try part1(data)});
    std.debug.print("Solution Problem 2: {d}\n", .{try part2(data)});
}

fn part1(data: [][]u8) !u32 {
    var x: usize = 0;
    var y: usize = 0;
    var result: u32 = 1;
    var dir: direction = .up;
    outer_loop: for (data, 0..) |line, j| {
        for (line, 0..) |pos, i| {
            if (pos == '^') {
                x = i;
                y = j;
                data[y][x] = 'X';
                break :outer_loop;
            }
        }
    }

    while (true) {
        if (x == data[0].len - 1 or y == data.len - 1 or x == 0 or y == 0) {
            break;
        }

        switch (dir) {
            .up => {
                if (data[y - 1][x] == '#') {
                    x += 1;
                    dir = .right;
                } else {
                    y -= 1;
                }
            },
            .down => {
                if (data[y + 1][x] == '#') {
                    x -= 1;
                    dir = .left;
                } else {
                    y += 1;
                }
            },
            .left => {
                if (data[y][x - 1] == '#') {
                    y -= 1;
                    dir = .up;
                } else {
                    x -= 1;
                }
            },
            .right => {
                if (data[y][x + 1] == '#') {
                    y += 1;
                    dir = .down;
                } else {
                    x += 1;
                }
            },
        }

        if (!(data[y][x] == 'X')) {
            data[y][x] = 'X';
            result += 1;
        }
    }

    return result;
}

fn part2(data: [][]const u8) !u32 {
    _ = data;
    return 0;
}

const test_alloc = std.testing.allocator;

test "Example input part1" {
    const string =
        \\....#.....
        \\.........#
        \\..........
        \\..#.......
        \\.......#..
        \\..........
        \\.#..^.....
        \\........#.
        \\#.........
        \\......#...
    ;

    const data = try format_input(string, test_alloc);
    defer {
        for (data) |da| {
            defer test_alloc.free(da);
        }
        test_alloc.free(data);
    }

    try std.testing.expectEqual(41, part1(data));
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
    defer {
        for (data) |da| {
            defer test_alloc.free(da);
        }
        test_alloc.free(data);
    }

    // try std.testing.expectEqual(9, part2(data));
}
