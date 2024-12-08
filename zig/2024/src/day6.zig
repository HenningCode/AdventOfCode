const std = @import("std");
const ArrList = std.ArrayList(u8);

const PointSet = std.AutoHashMapUnmanaged(Point, void);
const OrientedPoint = std.meta.Tuple(.{ Point, Direction });

const Direction = enum {
    up,
    down,
    left,
    right,

    fn next(self: @This()) Direction {
        switch (self) {
            .up => return .right,
            .down => return .left,
            .left => return .up,
            .right => return .down,
        }
    }
};

const Point = struct {
    x: i32,
    y: i32,

    const Self = @This();

    fn add(self: Self, other: Self) Self {
        return .{ .x = self.x + other.x, .y = self.y + other.y };
    }

    fn neighbor(self: Self, direction: Direction) Self {
        switch (direction) {
            .up => return self.add(.{ .x = 0, .y = -1 }),
            .down => return self.add(.{ .x = 0, .y = -1 }),
            .left => return self.add(.{ .x = -1, .y = 0 }),
            .right => return self.add(.{ .x = 1, .y = 0 }),
        }
    }

    fn isInBounds(self: Self, bound: Self) bool {
        return (self.x >= 0 and self.y >= 0 and self.y < bound.x and self.y < bound.y);
    }
};

const Guard = struct {
    position: Point,
    direction: Direction,
    area: Point,

    const Self = @This();

    fn move(self: Self, obstacle: PointSet) ?OrientedPoint {
        const peek = self.position.neighbor(self.direction);

        if (!self.position.isInBounds(peek)) {
            return null;
        } else if (obstacle.contains(peek)) {
            self.direction.next();
        } else {
            self.position = peek;
        }

        return .{ self.position, self.direction };
    }
};

    fn init(data: []const u8, alloc: std.mem.Allocator)   {

        var lines = std.mem.tokenizeAny(u8, data, "\n\r");
        while (lines.next()) |line| {
            len = line.len;
            for (line) |char| {
                if (char == 'X') {

                }
            }
        }

        return .{
            .data = arr,
            .row_len = len,
            .alloc = alloc,
        };
    }

const Grid = struct {
    data: ArrList,
    rows: usize,
    row_len: usize,
    start_x: usize,
    start_y: usize,
    alloc: std.mem.Allocator,

    const Self = @This();


    fn deinit(self: Self) void {
        self.data.deinit();
    }

    fn move(self: Self, grid: []u8, x: *usize, y: *usize, dir: *Direction) void {
        switch (dir.*) {
            .up => {
                if (grid[self.row_len * (y.* - 1) + x.*] == '#') {
                    x.* += 1;
                } else {
                    y.* -= 1;
                }
            },
            .down => {
                if (grid[self.row_len * (y.* + 1) + x.*] == '#') {
                    x.* -= 1;
                } else {
                    y.* += 1;
                }
            },
            .left => {
                if (grid[self.row_len * y.* + x.* - 1] == '#') {
                    y.* -= 1;
                } else {
                    x.* -= 1;
                }
            },
            .right => {
                if (grid[self.row_len * y.* + x.* + 1] == '#') {
                    y.* += 1;
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

        var obstacles = std.ArrayList(Point).init(self.alloc);
        defer obstacles.deinit();

        while (true) {
            self.move(data, &x, &y, &dir);
            if (x == row_len - 1 or y == rows - 1 or x == 0 or y == 0) {
                break;
            }
            const cur_pos = Point{ .x = x, .y = y };

            var found = false;
            for (obstacles.items) |pos| {
                if (pos.equal(cur_pos)) {
                    found = true;
                }
            }
            if (!found) {
                try obstacles.append(cur_pos);
            }
        }

        for (obstacles.items) |pos| {
            if (pos.equal(Point{
                .x = self.start_x,
                .y = self.start_y,
            })) {
                continue;
            }
            const obstruction_data = try self.data.clone();
            defer obstruction_data.deinit();
            obstruction_data.items[row_len * pos.y + pos.x] = '#';

            if (try self.checkIfLoop(obstruction_data.items, self.start_x, self.start_y, Direction.up)) {
                result += 1;
            }
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
        // std.debug.print("----------------------------------------------------------------------\n", .{});
        // std.debug.print("START\n", .{});
        // printGrid(grid, rows, row_len);

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
                    // std.debug.print("END\n", .{});
                    // printGrid(grid, rows, row_len);
                    // std.debug.print("----------------------------------------------------------------------\n", .{});
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

test "Bruh" {
    const sample =
        \\..#.............
        \\..............#.
        \\...#............
        \\........#.......
        \\................
        \\.......#.....#..
        \\................
        \\..^.............
    ;
    const data = try Grid.init(sample, test_alloc);

    defer data.deinit();

    try std.testing.expectEqual(1, data.part2());
}
