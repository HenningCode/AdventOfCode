const std = @import("std");

pub fn solution() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile("inputs/input2.txt", .{});
    defer file.close();
    const file_size = (try file.stat()).size;
    const file_contents = try file.reader().readAllAlloc(allocator, file_size);
    defer allocator.free(file_contents);

    std.debug.print("Day2\n", .{});
    std.debug.print("Solution Problem 1: {d}\n", .{try problem(file_contents, false, allocator)});
    std.debug.print("Solution Problem 2: {d}\n", .{try problem(file_contents, true, allocator)});
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

        const len: i32 = @intCast(number_list.items.len);
        const sum: i32 = sumArray(number_list);

        if (@divFloor(sum, len) < number_list.items[0] or @divFloor(sum, len) < number_list.items[1]) {
            inc = false;
        }

        var valid: bool = true;

        var i: usize = 0;
        var retry_flag = false;
        while (i < (number_list.items.len - 1)) {
            var difference: i32 = 0;
            if (inc) {
                difference = number_list.items[i + 1] - number_list.items[i];
            } else {
                difference = number_list.items[i] - number_list.items[i + 1];
            }

            if (difference > 3 or difference < 1) {
                if (dampener and !retry_flag) {
                    retry_flag = true;

                    // If the first element is already wrong just remove it and
                    // try again witht the next values
                    if (i == 0) {
                        _ = number_list.orderedRemove(i);
                        continue;
                    }
                    // Case if last element is wrong
                    if ((i + 2) >= number_list.items.len) break;

                    if (inc) {
                        difference = number_list.items[i + 2] - number_list.items[i];
                    } else {
                        difference = number_list.items[i] - number_list.items[i + 2];
                    }

                    if (difference < 1 or difference > 3) {
                        valid = false;
                        break;
                    }

                    _ = number_list.orderedRemove(i + 1);
                } else {
                    valid = false;
                }
            }
            i += 1;
        }
        if (valid) {
            result += 1;
        }
    }

    return result;
}

fn sumArray(array: std.ArrayList(i32)) i32 {
    var sum: i32 = 0;
    for (array.items) |x| {
        sum += x;
    }
    return sum;
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
    try std.testing.expectEqual(4, problem(input, true, std.testing.allocator));
}

test "Problem 2" {
    const input_2 =
        \\40 41 44 44 45 46 46
        \\68 70 71 70 72 75 76 76
        \\64 63 66 67 70
        \\49 51 54 57 60 64
        \\47 51 54 57 60 62
    ;
    try std.testing.expectEqual(3, problem(input_2, true, std.testing.allocator));
}

test "Problem 3" {
    const input_2 =
        \\47 51 54 57 60 62
        \\47 51 55 57 60 62
    ;
    try std.testing.expectEqual(1, problem(input_2, true, std.testing.allocator));
}
