use grid::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = solve(&input);
    println!("{result}");
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Right,
    Down,
    Up,
    Left,
}

#[derive(Debug, Clone, PartialEq)]
struct Ray {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ray {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }
}

fn solve(input: &str) -> u64 {
    let mut grid = Grid::new(0, 0);
    for line in input.lines() {
        grid.push_row(line.chars().collect::<Vec<char>>());
    }
    let directions = vec![
        Direction::Right,
        Direction::Left,
        Direction::Down,
        Direction::Up,
    ];
    let mut scores: Vec<u64> = Vec::new();

    for dir in directions {
        for j in 0..grid.cols() {

            let mut rays = match dir {
                Direction::Down => {vec![Ray::new(j as i32, 0, Direction::Down)]}
                Direction::Left => {vec![Ray::new(0, j as i32, Direction::Left)]}
                Direction::Right => {vec![Ray::new(0, j as i32, Direction::Right)]}
                Direction::Up => {vec![Ray::new(j as i32, 0, Direction::Up)]}
            };
            let mut grid_energy: Grid<u8> = Grid::new(grid.rows(), grid.cols());
            let mut seen_vec: Vec<(i32, i32)> = Vec::new();

            while !rays.is_empty() {
                for i in 0..rays.len() {
                    if rays[i].x < 0 || rays[i].x > grid.cols() as i32 - 1 {
                        rays.remove(i);
                        break;
                    }
                    if rays[i].y < 0 || rays[i].y > grid.rows() as i32 - 1 {
                        rays.remove(i);
                        break;
                    }

                    grid_energy[(rays[i].y as usize, rays[i].x as usize)] |= 1;

                    match grid[(rays[i].y as usize, rays[i].x as usize)] {
                        '/' => match rays[i].direction {
                            Direction::Down => {
                                rays[i].x -= 1;
                                rays[i].direction = Direction::Left;
                            }
                            Direction::Up => {
                                rays[i].x += 1;
                                rays[i].direction = Direction::Right;
                            }
                            Direction::Left => {
                                rays[i].y += 1;
                                rays[i].direction = Direction::Down;
                            }
                            Direction::Right => {
                                rays[i].y -= 1;
                                rays[i].direction = Direction::Up;
                            }
                        },
                        '\\' => match rays[i].direction {
                            Direction::Down => {
                                rays[i].x += 1;
                                rays[i].direction = Direction::Right;
                            }
                            Direction::Up => {
                                rays[i].x -= 1;
                                rays[i].direction = Direction::Left;
                            }
                            Direction::Left => {
                                rays[i].y -= 1;
                                rays[i].direction = Direction::Up;
                            }
                            Direction::Right => {
                                rays[i].y += 1;
                                rays[i].direction = Direction::Down;
                            }
                        },
                        '|' => match rays[i].direction {
                            Direction::Down => rays[i].y += 1,
                            Direction::Up => rays[i].y -= 1,
                            Direction::Left | Direction::Right => {
                                if !seen_vec.contains(&(rays[i].x, rays[i].y)) {
                                    seen_vec.push((rays[i].x, rays[i].y));
                                    rays.push(Ray::new(rays[i].x, rays[i].y + 1, Direction::Down));
                                    rays[i].direction = Direction::Up;
                                    rays[i].y -= 1;
                                } else {
                                    rays.remove(i);
                                    break;
                                }
                            }
                        },
                        '-' => match rays[i].direction {
                            Direction::Left => rays[i].x -= 1,
                            Direction::Right => rays[i].x += 1,
                            Direction::Up | Direction::Down => {
                                if !seen_vec.contains(&(rays[i].x, rays[i].y)) {
                                    seen_vec.push((rays[i].x, rays[i].y));
                                    rays.push(Ray::new(rays[i].x - 1, rays[i].y, Direction::Left));
                                    rays[i].direction = Direction::Right;
                                    rays[i].x += 1;
                                } else {
                                    rays.remove(i);
                                    break;
                                }
                            }
                        },
                        '.' => match rays[i].direction {
                            Direction::Down => rays[i].y += 1,
                            Direction::Up => rays[i].y -= 1,
                            Direction::Left => rays[i].x -= 1,
                            Direction::Right => rays[i].x += 1,
                        },
                        _ => panic!("Not found char!"),
                    }
                }
            }

            // print_gird(&grid);
            // 
            scores.push(calc_energy(&grid_energy));
        }
    }
    *scores.iter().max().unwrap()
}

#[allow(dead_code)]
fn print_gird<T: std::fmt::Display>(grid: &Grid<T>) {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            print!("{}", grid[(y, x)]);
        }
        println!();
    }
    println!()
}

fn calc_energy(grid: &Grid<u8>) -> u64 {
    let mut result = 0;
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid[(y, x)] == 1 {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        assert_eq!(solve(input), 46);
    }
}
