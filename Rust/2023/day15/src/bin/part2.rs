use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = solve(&input);
    println!("{result}");
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal: u8,
}

impl Lens {
    fn new(label: String, focal: u8) -> Self {
        Self { label, focal }
    }
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

fn calc_hash(str: &str) -> u8 {
    let mut current_value = 0;
    for char in str.chars() {
        current_value += char as u64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value as u8
}

fn solve(input: &str) -> u64 {
    let mut map: HashMap<u8, Vec<Lens>> = HashMap::new();
    let mut result = 0;

    for split in input.split(',') {
        if let Some((add, focals)) = split.split_once('=') {
            let label = calc_hash(add);
            let new_lens = Lens::new(add.to_string(), focals.parse::<u8>().unwrap());
            map.entry(label)
                .and_modify(|x| {
                    if x.contains(&new_lens) {
                        for lens in x {
                            if *lens == new_lens {
                                lens.focal = new_lens.focal;
                            }
                        }
                    } else {
                        x.push(new_lens.clone());
                    }
                })
                .or_insert(vec![new_lens]);
        } else {
            let (remove, _) = split.split_once('-').unwrap();
            let label = calc_hash(remove);
            let temp_lens = Lens::new(remove.to_string(), 0);
            map.entry(label).and_modify(|x| {
                if x.contains(&temp_lens) {
                    for (i, lens) in x.clone().iter_mut().enumerate() {
                        if *lens == temp_lens {
                            x.remove(i);
                            break;
                        }
                    }
                }
            });
        }
    }

    for x in map {
        if !x.1.is_empty() {
            for (i, len) in x.1.iter().enumerate() {
                let mut temp = 1 + x.0 as u64;
                temp *= (i + 1) as u64 * len.focal as u64;
                result += temp;
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(solve(input), 145);
    }
}
