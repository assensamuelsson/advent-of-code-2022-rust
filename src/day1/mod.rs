use super::DayResults;

pub fn run(contents: &String) -> DayResults {
    DayResults {
        part1: part1(contents),
        part2: Some(part2(contents)),
    }
}

fn parse_inventories(contents: &str) -> Vec<Vec<u128>> {
    let mut inventory: Vec<Vec<u128>> = Vec::new();
    let mut current = 0;

    for line in contents.lines() {
        if line.is_empty() {
            current += 1;
        }

        if current == inventory.len() {
            inventory.push(Vec::new());
        }

        if let Ok(n) = line.parse() {
            inventory[current].push(n);
        }
    }

    inventory
}

fn part1(contents: &str) -> u128 {
    parse_inventories(contents)
        .iter()
        .map(|i| i.iter().sum())
        .max()
        .unwrap()
}

fn part2(contents: &str) -> u128 {
    let mut sums: Vec<u128> = parse_inventories(contents)
        .iter()
        .map(|i| i.iter().sum())
        .collect();

    sums.sort();
    sums.reverse();

    sums
        .iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn should_parse_inventories_correctly() {
        let inventories = parse_inventories(EXAMPLE_INPUT);

        assert_eq!(inventories, vec![
            vec![1000u128, 2000u128, 3000u128],
            vec![4000u128],
            vec![5000u128, 6000u128],
            vec![7000u128, 8000u128, 9000u128],
            vec![10000u128],
        ]);
    }

    #[test]
    fn should_calculate_part1_correctly() {
        let result = part1(EXAMPLE_INPUT);

        assert_eq!(result, 24000u128);
    }
    
    #[test]
    fn should_calculate_part2_correctly() {
        let result = part2(EXAMPLE_INPUT);

        assert_eq!(result, 45000);
    }
}