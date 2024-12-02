use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Above,
    Below,
}

fn find_starting_point(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 'S' {
                return (y, x);
            }
        }
    }
    (0, 0)
}

fn checked_sub(value: usize, rh: usize) -> Option<usize> {
    value.checked_sub(rh)
}

fn check_add_max(value: usize, rh: usize, max: usize) -> Option<usize> {
    if (value + rh) > max {
        return None;
    }
    Some(value + rh)
}

fn find_starting_pipes(
    map: &Vec<Vec<char>>,
    coords: (usize, usize),
) -> ((usize, usize, Direction), (usize, usize, Direction)) {
    let above = (checked_sub(coords.0, 1), coords.1);
    let below = (check_add_max(coords.0, 1, map.len()), coords.1);
    let right = (coords.0, check_add_max(coords.1, 1, map[0].len()));
    let left = (coords.0, checked_sub(coords.1, 1));
    let mut new_direction = Vec::new();

    if above.0.is_some() {
        let char = map[above.0.unwrap()][above.1];
        match char {
            '|' => new_direction.push((above.0.unwrap(), above.1, Direction::Above)),
            '7' => new_direction.push((above.0.unwrap(), above.1, Direction::Left)),
            'F' => new_direction.push((above.0.unwrap(), above.1, Direction::Right)),
            _ => {}
        }
    }

    if below.0.is_some() {
        let char = map[below.0.unwrap()][below.1];
        match char {
            '|' => new_direction.push((below.0.unwrap(), below.1, Direction::Below)),
            'L' => new_direction.push((below.0.unwrap(), below.1, Direction::Right)),
            'J' => new_direction.push((below.0.unwrap(), below.1, Direction::Left)),
            _ => {}
        }
    }

    if right.1.is_some() {
        let char = map[right.0][right.1.unwrap()];
        match char {
            '-' => new_direction.push((right.0, right.1.unwrap(), Direction::Right)),
            '7' => new_direction.push((right.0, right.1.unwrap(), Direction::Below)),
            'J' => new_direction.push((right.0, right.1.unwrap(), Direction::Above)),
            _ => {}
        }
    }

    if left.1.is_some() {
        let char = map[left.0][left.1.unwrap()];
        match char {
            '-' => new_direction.push((right.0, right.1.unwrap(), Direction::Left)),
            'F' => new_direction.push((left.0, left.1.unwrap(), Direction::Below)),
            'L' => new_direction.push((left.0, left.1.unwrap(), Direction::Above)),
            _ => {}
        }
    }

    if new_direction.len() == 2 {
        return (new_direction[0], new_direction[1]);
    }
    panic!("More than two connected pipes found");
}

fn get_next_coord(
    coords: (usize, usize, Direction),
    map: &Vec<Vec<char>>,
) -> (usize, usize, Direction) {

    let new_coord = match coords.2 {
        Direction::Above => (coords.0 - 1, coords.1),
        Direction::Below => (coords.0 + 1, coords.1),
        Direction::Left => (coords.0, coords.1 - 1),
        Direction::Right => (coords.0, coords.1 + 1),
    };

    let new_direction = match map[new_coord.0][new_coord.1] {
        '|' => match coords.2 {
            Direction::Below => Direction::Below,
            Direction::Above => Direction::Above,
            _ => panic!("Not possible")
        }
        '-' => match coords.2 {
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
            _ => panic!("Not possible")
        }
        'L' => match coords.2 {
            Direction::Below => Direction::Right,
            Direction::Left => Direction::Above,
            _ => panic!("Not possible")
        }
        'J' => match coords.2 {
            Direction::Below => Direction::Left,
            Direction::Right => Direction::Above,
            _ => panic!("Not possible")
        }
        '7' => match coords.2 {
            Direction::Right => Direction::Below,
            Direction::Above => Direction::Left,
            _ => panic!("Not possible")
        }
        'F' => match coords.2 {
            Direction::Above => Direction::Right,
            Direction::Left => Direction::Below,
            _ => panic!("Not possible")
        }

        _ => panic!("Unknown Char")
    };

    (new_coord.0, new_coord.1, new_direction)
}

fn parse_input(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let starting_coords = find_starting_point(&map);

    let (mut path1, mut path2) = find_starting_pipes(&map, starting_coords);

    let mut i = 1;

    loop {
        path1 = get_next_coord(path1, &map);
        path2 = get_next_coord(path2, &map);

        i += 1;
        if path1.0 == path2.0 && path1.1 == path2.1 {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(parse_input(input), 8)
    }

    #[test]
    fn test_check_add() {
        assert_eq!(check_add_max(12, 1, 20), Some(13));
        assert_eq!(check_add_max(19, 1, 20), Some(20));
        assert_eq!(check_add_max(20, 1, 20), None);
    }
}
