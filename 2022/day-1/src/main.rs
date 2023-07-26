use std::collections::HashMap;
use std::fs;

pub fn separate_elves(string: &str) -> HashMap<i32, i32> {
    let all_items = string.split('\n');

    let mut index: i32 = 0;
    let mut elfs_items = HashMap::<i32, i32>::new();

    for item in all_items {
        if item.is_empty() && elfs_items.len() > 0 {
            index += 1;
            continue;
        }

        let callories = match str::parse::<i32>(item) {
            Ok(it) => it,
            Err(_err) => continue,
        };

        if elfs_items.contains_key(&index) {
            elfs_items.entry(index).and_modify(|current_callories| *current_callories += callories);
        } else {
            elfs_items.insert(index, callories);
        }
    }

    return elfs_items;
}

pub fn get_greedy_elves(elves: &HashMap<i32, i32>, count: usize) -> Vec<(i32, i32)> {
    let mut elves_vec: Vec<_> = elves.iter().map(|x| (*x.0, *x.1)).collect();

    elves_vec.sort_by(|current, compare| current.1.cmp(&compare.1));

    let greediest_elves = elves_vec[elves_vec.len() - count..elves_vec.len()].to_vec();

    return greediest_elves;
}

pub fn total_callories(greedy_elves: &Vec<(i32, i32)>) -> i32 {
    let mut total = 0;

    for elf in greedy_elves {
        total += elf.1;
    }

    return total;
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Error reading input.");

    let result = separate_elves(&input);

    let greedy_elves = get_greedy_elves(&result, 3);

    let total = total_callories(&greedy_elves);

    println!();
    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_the_greediest_elves() {
        let input = fs::read_to_string("./input.txt").expect("Error reading input.");

        let result = separate_elves(&input);
        
        let greedy_elves = get_greedy_elves(&result, 3);

        let total = total_callories(&greedy_elves);

        dbg!(total);

        assert!(total == 201524);
    }
}
