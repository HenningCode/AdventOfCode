const std = @import("std");
const ArrList = std.ArrayList(u8);

const PointSet = std.AutoArrayHashMapUnmanaged(Point, void);
const OrientedPoint = struct { Point, Direction };
const OrientedPointSet = std.AutoHashMapUnmanaged(OrientedPoint, void);
const StartCondition = struct { Guard, PointSet };

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
            .down => return self.add(.{ .x = 0, .y = 1 }),
            .left => return self.add(.{ .x = -1, .y = 0 }),
            .right => return self.add(.{ .x = 1, .y = 0 }),
        }
    }

    fn isInBounds(self: Self, bound: Self) bool {
        return (self.x >= 0 and self.y >= 0 and self.x < bound.x and self.y < bound.y);
    }
};

const Guard = struct {
    position: Point,
    direction: Direction,
    area: Point,

    const Self = @This();

    fn move(self: *Self, obstacle: PointSet) ?OrientedPoint {
        const peek = self.position.neighbor(self.direction);
        if (!peek.isInBounds(self.area)) {
            return null;
        } else if (obstacle.contains(peek)) {
            self.direction = self.direction.next();
        } else {
            self.position = peek;
        }

        return .{ self.position, self.direction };
    }
};

fn init(data: []const u8, alloc: std.mem.Allocator) !StartCondition {
    var obstacles: PointSet = .empty;
    var guard: ?Point = null;

    var lines = std.mem.tokenizeAny(u8, data, "\n\r");
    var y: i32 = 0;
    var x: i32 = 0;
    while (lines.next()) |line| {
        x = @intCast(line.len);
        for (line, 0..) |char, i| {
            if (char == '#') {
                try obstacles.put(alloc, .{ .x = @intCast(i), .y = y }, {});
            } else if (char == '^') {
                guard = .{ .x = @intCast(i), .y = y };
            }
        }
        y += 1;
    }

    return .{
        Guard{
            .position = guard.?,
            .direction = Direction.up,
            .area = .{ .x = x, .y = y },
        },
        obstacles,
    };
}

fn part1(alloc: std.mem.Allocator, guard: Guard, obstacles: PointSet) !u32 {
    var visited: PointSet = .empty;
    defer visited.deinit(alloc);
    var patrol = guard;

    while (patrol.move(obstacles)) |pos| {
        try visited.put(alloc, pos[0], {});
    }

    return @intCast(visited.count());
}

fn part2(alloc: std.mem.Allocator, guard: Guard, obstacles: PointSet) !u32 {
    var new_obstacles: PointSet = .empty;
    defer new_obstacles.deinit(alloc);

    var loops: u32 = 0;
    var patrol = guard;
    const start_pos: OrientedPoint = .{ patrol.position, patrol.direction };

    while (patrol.move(obstacles)) |pos| {
        try new_obstacles.put(alloc, pos[0], {});
    }
    // remove the start position
    _ = new_obstacles.orderedRemove(start_pos[0]);

    for (new_obstacles.keys()) |new_obstacle| {
        var alt_guard = Guard{
            .position = start_pos[0],
            .direction = Direction.up,
            .area = guard.area,
        };

        var alt_obs = try obstacles.clone(alloc);
        try alt_obs.put(alloc, new_obstacle, {});
        defer alt_obs.deinit(alloc);

        var visited: OrientedPointSet = .empty;
        defer visited.deinit(alloc);

        while (alt_guard.move(alt_obs)) |pos| {
            if (visited.contains(pos)) {
                loops += 1;
                break;
            }
            try visited.put(alloc, pos, {});
        }
    }

    return loops;
}

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const file = @embedFile("inputs/input6.txt");
    const guard, var obstacles = try init(file, alloc);
    defer obstacles.deinit(alloc);

    std.debug.print("Day6\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try part1(alloc, guard, obstacles)});
    std.debug.print("Solution Problem 2: {d}\n", .{try part2(alloc, guard, obstacles)});
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

    const guard, var obstacles = try init(string, test_alloc);
    defer obstacles.deinit(test_alloc);
    try std.testing.expectEqual(41, part1(test_alloc, guard, obstacles));
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
    const guard, var obstacles = try init(string, test_alloc);
    defer obstacles.deinit(test_alloc);
    try std.testing.expectEqual(6, part2(test_alloc, guard, obstacles));
}

test "Bruh" {
    const string =
        \\..#.............
        \\..............#.
        \\...#............
        \\........#.......
        \\................
        \\.......#.....#..
        \\................
        \\..^.............
    ;
    const guard, var obstacles = try init(string, test_alloc);
    defer obstacles.deinit(test_alloc);
    try std.testing.expectEqual(1, part2(test_alloc, guard, obstacles));
}
