const std = @import("std");

pub fn solution() !void {
    std.debug.print("Day3\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try part1(@embedFile("inputs/input3.txt"))});
    std.debug.print("Solution Problem 2: {d}\n", .{try part2(@embedFile("inputs/input3.txt"))});
}

fn part1(data: []const u8) !u32 {
    var instructions = std.mem.tokenizeSequence(u8, data, "mul(");
    var result: u32 = 0;
    while (instructions.next()) |x| {
        var numbers = std.mem.tokenizeScalar(u8, x, ',');
        const num1 = numbers.next();
        var num2_it = std.mem.tokenizeScalar(u8, numbers.rest(), ')');
        const num2 = num2_it.next();

        const n1 = std.fmt.parseInt(u32, num1.?, 10) catch continue;
        const n2 = std.fmt.parseInt(u32, num2.?, 10) catch continue;

        result += n1 * n2;
    }
    return result;
}

fn part2(data: []const u8) !u32 {
    var result: u32 = 0;
    var instructions = std.mem.tokenizeSequence(u8, data, "don't()");

    while (instructions.peek() != null) {
        result += try part1(instructions.next().?);
        instructions = std.mem.tokenizeSequence(u8, instructions.rest(), "do()");
        _ = instructions.next();
        instructions = std.mem.tokenizeSequence(u8, instructions.rest(), "don't()");
    }

    return result;
}

test "Example input" {
    const string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    try std.testing.expectEqual(161, part1(string));
}

test "Exampl input problem 2" {
    const string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)+mul(32,64](mul(11,8)undo()mul(11,8)";
    try std.testing.expectEqual(48 * 3 + 88, part2(string));
}
