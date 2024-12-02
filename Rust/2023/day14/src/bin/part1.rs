use grid::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = solve(&input);
    println!("{result}");
}

fn solve(input: &str) -> u64 {
    let mut data_grid: Grid<char> = Grid::new(0, 0);

    for line in input.lines() {
        data_grid.push_row(line.chars().collect::<Vec<char>>());
    }
    for y in 0..data_grid.rows() {
        for x in 0..data_grid.cols() {
            if data_grid[(y, x)] == 'O' {
                move_up(&mut data_grid, y, x);
            }
        }
    }

    print_grid(&data_grid);
    calc_result(&data_grid)
}

fn calc_result(grid: &Grid<char>) -> u64 {
    let mut result = 0;

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid[(y, x)] == 'O' {
                result += grid.rows() - y;
            }
        }
    }

    result as u64
}

fn move_up(grid: &mut Grid<char>, y: usize, x: usize) {
    for i in (0..y).rev() {
        if grid[(i, x)] == '.' {
            grid[(i + 1, x)] = '.';
            grid[(i, x)] = 'O';
        } else {
            break;
        }
    }
}

fn print_grid(gird: &Grid<char>) {
    for y in 0..gird.rows() {
        for x in 0..gird.cols() {
            print!("{}", gird[(y, x)]);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        assert_eq!(solve(input), 136);
    }
}
