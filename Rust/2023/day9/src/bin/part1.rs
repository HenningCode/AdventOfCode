use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn get_number_from_line(input: &str) -> Vec<i64> {
    let mut number_vec = Vec::new();

    for number in input.trim().split(' ') {
        if let Ok(number) = number.parse() {
            number_vec.push(number)
        };
    }
    number_vec
}

fn calc_rows(input: Vec<i64>) -> Vec<Vec<i64>> {
    let mut row_vec: Vec<Vec<i64>> = vec![input];
    loop {
        let current_row = row_vec.last().unwrap();
        let mut new_line = Vec::new();
        for i in 0..current_row.len() - 1 {
            new_line.push(current_row[i + 1] - current_row[i])
        }
        row_vec.push(new_line.clone());
        if new_line.iter().all(|&x|x == 0) {
            break;
        }
    }
    row_vec
}

fn calc_continuation(row_matix: &mut Vec<Vec<i64>>) -> i64 {
    let mut first_row = true;
    let mut last_row: &mut Vec<i64> = &mut Vec::new();
    let mut result = 0;

    for (i, row) in &mut row_matix.iter_mut().enumerate().rev() {
        if first_row {
            first_row = false;
            (*row).push(0);
            last_row = row;
            continue;
        }

        if i == 0 {
            result = last_row.last().unwrap() + row.last().unwrap();
        }
        (*row).push(last_row.last().unwrap() + row.last().unwrap());
        last_row = row;
    }
    println!("{:?}", row_matix);
    result
}

fn parse_input(input: &str) -> i128 {
    let mut result = 0;

    for line in input.lines() {
        let nums = get_number_from_line(line);
        let mut rows = calc_rows(nums);
        result += calc_continuation(&mut rows) as i128;
    }

    result 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_input() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
0 -1 -1 0";

        assert_eq!(parse_input(input), 116);
    }
}
