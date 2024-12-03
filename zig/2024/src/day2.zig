const std = @import("std");

const Order = enum {
    unkown,
    descending,
    ascending,
};

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    std.debug.print("Day2\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try new_solution(@embedFile("inputs/input2.txt"), false, allocator)});
    std.debug.print("Solution Problem 2: {d}\n", .{try new_solution(@embedFile("inputs/input2.txt"), true, allocator)});
}

fn problem(data: []const u8, dampener: bool, allocator: std.mem.Allocator) !usize {
    var lines = std.mem.tokenizeAny(u8, data, "\n");
    var result: usize = 0;

    while (lines.next()) |line| {
        var numbers = std.mem.tokenizeAny(u8, line, " ");
        var number_list = std.ArrayList(i32).init(allocator);
        defer number_list.deinit();

        while (numbers.next()) |number| {
            try number_list.append(try std.fmt.parseInt(i32, number, 10));
        }

        var inc: bool = true;
        var valid: bool = undefined;

        // Brute force increasing or decreasing list
        for (0..2) |_| {
            inc = !inc;
            valid = true;

            var i: usize = 0;
            var retry_flag = false;

            var temp_list = try number_list.clone();
            defer temp_list.deinit();

            while (i < (temp_list.items.len - 1)) {
                var difference: i32 = 0;

                if (inc) {
                    difference = temp_list.items[i + 1] - temp_list.items[i];
                } else {
                    difference = temp_list.items[i] - temp_list.items[i + 1];
                }

                if (difference > 3 or difference < 1) {
                    if (dampener and !retry_flag) {
                        retry_flag = true;

                        // Case if last element is wrong -> its working
                        if ((i + 2) >= temp_list.items.len) break;

                        if (i == 0) {
                            // If the first element is already wrong just remove it and
                            // try again with the next values
                            _ = temp_list.orderedRemove(i);
                        } else {
                            // remove the wrong element and retry
                            _ = temp_list.orderedRemove(i + 1);
                        }

                        // Dont increase i to retry on the current value without the wrong value
                        continue;
                    } else {
                        valid = false;
                        break;
                    }
                }
                i += 1;
            }
            if (valid) {
                result += 1;
                break;
            }
        }
    }

    return result;
}

fn new_solution(data: []const u8, dampener: bool, allocator: std.mem.Allocator) !usize {
    var lines = std.mem.tokenizeAny(u8, data, "\n");
    var result: usize = 0;

    new_list: while (lines.next()) |line| {
        var numbers = std.mem.tokenizeAny(u8, line, " ");
        var number_list = std.ArrayList(i32).init(allocator);
        defer number_list.deinit();

        while (numbers.next()) |number| {
            try number_list.append(try std.fmt.parseInt(i32, number, 10));
        }

        if (check_list(number_list)) {
            result += 1;
            continue;
        }

        if (dampener) {
            for (0..number_list.items.len) |i| {
                var modified_list = try number_list.clone();
                defer modified_list.deinit();
                _ = modified_list.orderedRemove(i);
                if (check_list(modified_list)) {
                    result += 1;
                    continue :new_list;
                }
            }
        }
    }

    return result;
}

fn check_list(list: std.ArrayList(i32)) bool {
    var order: Order = .unkown;
    var prev_n = list.items[0];
    for (list.items, 0..) |n, i| {
        switch (order) {
            .unkown => {
                if (i > 1) return false;
                if (@abs(n - prev_n) > 3) return false;
                order = if (n - prev_n > 0) .ascending else if (prev_n - n > 0) .descending else .unkown;
            },
            .ascending => {
                if (n - prev_n > 3 or n - prev_n < 1) return false;
            },
            .descending => {
                if (prev_n - n > 3 or prev_n - n < 1) return false;
            },
        }
        prev_n = n;
    }
    return true;
}

test "Problem 1" {
    const input =
        \\7 6 4 2 1
        \\1 2 7 8 9
        \\9 7 6 2 1
        \\1 3 2 4 5
        \\8 6 4 4 1
        \\1 3 6 7 9
    ;

    try std.testing.expectEqual(2, problem(input, false, std.testing.allocator));
    try std.testing.expectEqual(4, new_solution(input, std.testing.allocator));
}

test "Problem 2" {
    const input_2 =
        \\40 41 44 44 45 46 46
        \\68 70 71 70 72 75 76 76
        \\64 63 66 67 70
        \\49 51 54 57 60 64
        \\47 51 54 57 60 62
    ;
    try std.testing.expectEqual(3, new_solution(input_2, std.testing.allocator));
}

test "Problem 3" {
    const input_2 =
        \\47 51 54 57 60 62
        \\47 51 55 57 60 62
        \\6 2 3 4 5 6
    ;
    try std.testing.expectEqual(2, new_solution(input_2, std.testing.allocator));
}
