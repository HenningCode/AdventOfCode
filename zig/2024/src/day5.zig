const std = @import("std");

fn parseU32(data: []const u8) !u32 {
    return try std.fmt.parseInt(u32, data, 10);
}

const Data = struct {
    rules: [][2]u32,
    instructions: [][]u32,

    const Self = @This();

    pub fn init(data: []const u8, alloc: std.mem.Allocator) !Data {
        var rules = std.ArrayList([2]u32).init(alloc);

        var instructions = std.ArrayList([]u32).init(alloc);
        defer instructions.deinit();

        var blocks = std.mem.tokenizeSequence(u8, data, "\n\n");

        var lines = std.mem.tokenizeScalar(u8, blocks.next().?, '\n');
        while (lines.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " ");
            var rule = std.mem.splitScalar(u8, trimmed, '|');
            try rules.append(.{ try parseU32(rule.first()), try parseU32(rule.rest()) });
        }

        var lines_inst = std.mem.tokenizeScalar(u8, blocks.next().?, '\n');
        while (lines_inst.next()) |line| {
            const trimmed = std.mem.trim(u8, line, " ");
            var single_instruction = std.ArrayList(u32).init(alloc);
            var single_val = std.mem.splitScalar(u8, trimmed, ',');

            while (single_val.next()) |x| {
                try single_instruction.append(try parseU32(x));
            }

            try instructions.append(try single_instruction.toOwnedSlice());
        }

        return .{
            .rules = try rules.toOwnedSlice(),
            .instructions = try instructions.toOwnedSlice(),
        };
    }

    pub fn deinit(self: Self, alloc: std.mem.Allocator) void {
        for (self.instructions) |ins| {
            alloc.free(ins);
        }
        alloc.free(self.instructions);
        alloc.free(self.rules);
    }

    pub fn part1(self: Self) !u32 {
        var result: u32 = 0;
        for (self.instructions) |ins| {
            if (self.check_rules(ins)) {
                result += try Data.get_middle_value(ins);
            }
        }
        return result;
    }

    pub fn part2(self: Self) !u32 {
        var result: u32 = 0;
        for (self.instructions) |ins| {
            if (!self.check_rules(ins)) {
                result += try self.get_middle_of_fixed_array(ins);
            }
        }
        return result;
    }

    fn get_middle_of_fixed_array(self: Self, ins: []u32) !u32 {
        var arr = ins;
        var i: usize = 0;

        while (true) {
            if (self.check_rules(arr)) {
                return try Data.get_middle_value(arr);
            } else {
                if (!self.check_single_pair(arr[i], arr[i + 1])) {
                    const tmp = arr[i];
                    arr[i] = arr[i + 1];
                    arr[i + 1] = tmp;

                    if (i > 0) {
                        i -= 1;
                    }
                    continue;
                }
            }
            i += 1;
        }
    }

    fn check_single_pair(self: Self, x: u32, y: u32) bool {
        for (self.rules) |rule| {
            if (rule[0] == x and rule[1] == y) {
                return true;
            }
        }
        return false;
    }

    fn check_rules(self: Self, arr: []u32) bool {
        for (arr, 0..) |value, i| {
            rule_loop: for (i + 1..arr.len) |j| {
                for (self.rules) |rule| {
                    if (rule[0] == value and rule[1] == arr[j]) {
                        continue :rule_loop;
                    }
                }
                return false;
            }
        }
        return true;
    }

    fn get_middle_value(arr: []u32) !u32 {
        const middle_idx = try std.math.divTrunc(u32, @intCast(arr.len), 2);
        return arr[middle_idx];
    }
};

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    const data = try Data.init(@embedFile("inputs/input5.txt"), alloc);
    defer data.deinit(alloc);

    std.debug.print("Day5\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try data.part1()});
    std.debug.print("Solution Problem 2: {d}\n", .{try data.part2()});
}

fn part1(data: []const u8) !u32 {
    _ = data;
    return 0;
}

fn part2(data: []const u8) !u32 {
    _ = data;
    return 0;
}

test "Example input part1" {
    const string =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
    ;

    const alloc = std.testing.allocator;
    const data = try Data.init(string, alloc);
    defer data.deinit(alloc);

    try std.testing.expectEqual(143, data.part1());
}

test "Example input part2" {
    const string =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
    ;

    const alloc = std.testing.allocator;
    const data = try Data.init(string, alloc);
    defer data.deinit(alloc);

    try std.testing.expectEqual(123, data.part2());
}
