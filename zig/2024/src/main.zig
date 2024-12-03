const day1 = @import("day1.zig");
const day2 = @import("day2.zig");
const day3 = @import("day3.zig");

pub fn main() !void {
    try day1.solution();
    try day2.solution();
    try day3.solution();
}