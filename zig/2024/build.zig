const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const get_inputs = b.addExecutable(.{
        .name = "get_input",
        .root_source_file = b.path("src/get_inputs.zig"),
        .target = target,
        .optimize = optimize,
    });

    b.installArtifact(get_inputs);

    const day1 = b.addExecutable(.{
        .name = "aoc",
        .root_source_file = b.path("src/day1.zig"),
        .target = target,
        .optimize = optimize,
    });
    b.installArtifact(day1);

    const run_cmd = b.addRunArtifact(day1);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_cmd_inputs = b.addRunArtifact(get_inputs);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("day1", "Run puzzle day1");
    run_step.dependOn(&run_cmd.step);

    const run_step_inputs = b.step("get_inputs", "Get inputs for the day");
    run_step_inputs.dependOn(&run_cmd_inputs.step);
}
