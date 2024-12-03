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

                        // Case if last element is wrong its working
                        if ((i + 2) >= temp_list.items.len) break;

                        if (i == 0) {
                            // If the first element is already wrong just remove it and
                            // try again witht the next values
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
        \\6 2 3 4 5 6
    ;
    try std.testing.expectEqual(2, problem(input_2, true, std.testing.allocator));
}
