const std = @import("std");
const ArrList = std.ArrayList(u8);

const Direction = enum {
    up,
    down,
    left,
    right,
};

fn printGrid(grid: []u8, rows: usize, row_len: usize) void {
    for (0..rows) |j| {
        for (0..row_len) |i| {
            std.debug.print("{c}", .{grid[j * row_len + i]});
        }
        std.debug.print("\n", .{});
    }
    std.debug.print("\n", .{});
}

const Grid = struct {
    data: ArrList,
    rows: usize,
    row_len: usize,
    start_x: usize,
    start_y: usize,
    alloc: std.mem.Allocator,

    const Self = @This();

    fn init(data: []const u8, alloc: std.mem.Allocator) !Self {
        var arr = ArrList.init(alloc);
        var len: usize = 0;

        var lines = std.mem.tokenizeAny(u8, data, "\n\r");
        while (lines.next()) |line| {
            len = line.len;
            for (line) |char| {
                try arr.append(char);
            }
        }

        var x: usize = 0;
        var y: usize = 0;
        const rows = arr.items.len / len;
        outer_loop: for (0..rows) |j| {
            for (0..rows) |i| {
                if (arr.items[j * len + i] == '^') {
                    x = i;
                    y = j;
                    arr.items[y * len + x] = 'X';
                    break :outer_loop;
                }
            }
        }
        return .{
            .data = arr,
            .rows = rows,
            .row_len = len,
            .start_x = x,
            .start_y = y,
            .alloc = alloc,
        };
    }

    fn deinit(self: Self) void {
        self.data.deinit();
    }

    fn move(self: Self, grid: []u8, x: *usize, y: *usize, dir: *Direction) void {
        switch (dir.*) {
            .up => {
                if (grid[self.row_len * (y.* - 1) + x.*] == '#' or grid[self.row_len * (y.* - 1) + x.*] == 'O') {
                    // Needed if there are two obstacles next to each other
                    if (grid[self.row_len * y.* + x.* + 1] == '#' or grid[self.row_len * y.* + x.* + 1] == 'O') {
                        y.* += 1;
                        dir.* = .down;
                    } else {
                        x.* += 1;
                        dir.* = .right;
                    }
                } else {
                    y.* -= 1;
                }
            },
            .down => {
                if (grid[self.row_len * (y.* + 1) + x.*] == '#' or grid[self.row_len * (y.* + 1) + x.*] == 'O') {
                    // Needed if there are two obstacles next to each other
                    if (grid[self.row_len * y.* + x.* - 1] == '#' or grid[self.row_len * y.* + x.* - 1] == 'O') {
                        y.* -= 1;
                        dir.* = .up;
                    } else {
                        x.* -= 1;
                        dir.* = .left;
                    }
                } else {
                    y.* += 1;
                }
            },
            .left => {
                if (grid[self.row_len * y.* + x.* - 1] == '#' or grid[self.row_len * y.* + x.* - 1] == 'O') {
                    // Needed if there are two obstacles next to each other
                    if (grid[self.row_len * (y.* - 1) + x.*] == '#' or grid[self.row_len * (y.* - 1) + x.*] == 'O') {
                        x.* += 1;
                        dir.* = .right;
                    } else {
                        y.* -= 1;
                        dir.* = .up;
                    }
                } else {
                    x.* -= 1;
                }
            },
            .right => {
                if (grid[self.row_len * y.* + x.* + 1] == '#' or grid[self.row_len * y.* + x.* + 1] == 'O') {
                    // Needed if there are two obstacles next to each other
                    if (grid[self.row_len * (y.* + 1) + x.*] == '#' or grid[self.row_len * (y.* + 1) + x.*] == 'O') {
                        x.* -= 1;
                        dir.* = .left;
                    } else {
                        y.* += 1;
                        dir.* = .down;
                    }
                } else {
                    x.* += 1;
                }
            },
        }
    }

    fn part1(self: Self) !u32 {
        const data_clone = try self.data.clone();
        defer data_clone.deinit();
        const data = data_clone.items;
        const rows = self.rows;
        const row_len = self.row_len;

        var x: usize = self.start_x;
        var y: usize = self.start_y;
        var result: u32 = 1;
        var dir: Direction = .up;

        while (true) {
            if (x == row_len - 1 or y == rows - 1 or x == 0 or y == 0) {
                break;
            }

            self.move(data, &x, &y, &dir);

            if (!(data[row_len * y + x] == 'X')) {
                data[row_len * y + x] = 'X';
                result += 1;
            }
        }

        return result;
    }

    fn part2(self: Self) !u32 {
        const data_clone = try self.data.clone();
        defer data_clone.deinit();
        const data = data_clone.items;
        const rows = self.rows;
        const row_len = self.row_len;

        var x = self.start_x;
        var y = self.start_y;
        var result: u32 = 0;
        var dir: Direction = .up;

        while (true) {
            // check if left the grid
            if (x == row_len - 1 or y == rows - 1 or x == 0 or y == 0) {
                break;
            }

            // Place and try the obstacle
            const obstruction_data = try self.data.clone();
            defer obstruction_data.deinit();

            obstruction_data.items[row_len * self.start_y + self.start_x] = '.';
            obstruction_data.items[row_len * y + x] = 'X';
            switch (dir) {
                .up => {
                    obstruction_data.items[row_len * (y - 1) + x] = 'O';
                },
                .down => {
                    obstruction_data.items[row_len * (y + 1) + x] = 'O';
                },
                .left => {
                    obstruction_data.items[row_len * y + x - 1] = 'O';
                },
                .right => {
                    obstruction_data.items[row_len * y + x + 1] = 'O';
                },
            }

            if (try self.checkIfLoop(obstruction_data.items, x, y, dir)) {
                result += 1;
            }

            // move ahead
            self.move(data, &x, &y, &dir);
        }

        return result;
    }

    fn checkIfLoop(self: Self, grid: []u8, start_x: usize, start_y: usize, start_dir: Direction) !bool {
        var x = start_x;
        var y = start_y;
        var dir = start_dir;
        const rows = self.rows;
        const row_len = self.row_len;

        const Pos = struct {
            x: usize,
            y: usize,
            dir: Direction,

            fn equal(this: @This(), other: @This()) bool {
                if (this.x == other.x and this.y == other.y and this.dir == other.dir) {
                    return true;
                }
                return false;
            }
        };

        var positions = std.ArrayList(Pos).init(self.alloc);
        defer positions.deinit();

        while (true) {
            const current_pos = Pos{
                .x = x,
                .y = y,
                .dir = dir,
            };

            self.move(grid, &x, &y, &dir);
            grid[row_len * y + x] = 'X';

            // a postion was reached with same direction again
            for (positions.items) |pos| {
                if (pos.equal(current_pos)) {
                    return true;
                }
            }

            // The guard found an escape
            if (x == row_len - 1 or y == rows - 1 or x == 0 or y == 0) {
                return false;
            }

            try positions.append(current_pos);
        }
    }
};

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const file = @embedFile("inputs/input6.txt");
    const data = try Grid.init(file, alloc);
    defer data.deinit();

    std.debug.print("Day6\n", .{});

    std.debug.print("Solution Problem 1: {d}\n", .{try data.part1()});
    std.debug.print("Solution Problem 2: {d}\n", .{try data.part2()});
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

    const data = try Grid.init(string, test_alloc);
    defer data.deinit();

    try std.testing.expectEqual(41, data.part1());
}

test "Example input part2" {
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

    const data = try Grid.init(string, test_alloc);
    defer data.deinit();

    try std.testing.expectEqual(6, data.part2());
}
