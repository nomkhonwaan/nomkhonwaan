use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead},
    path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut parse_page_ordering_rules = true;
    let mut page_ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    parse_page_ordering_rules = false;
                    continue;
                }

                if parse_page_ordering_rules {
                    let s: Vec<&str> = line.split("|").collect();

                    page_ordering_rules
                        .entry(s[0].parse().unwrap())
                        .or_insert(vec![])
                        .push(s[1].parse().unwrap());
                } else {
                    updates.push(line.split(",").map(|s| s.parse().unwrap()).collect());
                }
            }
        }
    }

    println!(
        "First part answer is: {}",
        cal_first_part_answer(&page_ordering_rules, &updates)
    );
    println!(
        "Second part answer is: {}",
        cal_second_part_answer(&page_ordering_rules, &updates)
    );
}

fn cal_first_part_answer(
    page_ordering_rules: &HashMap<i32, Vec<i32>>,
    updates: &Vec<Vec<i32>>,
) -> i32 {
    let mut total_middle_page_number = 0;

    for update in updates {
        if is_in_the_right_order(page_ordering_rules, update) {
            total_middle_page_number += find_middle_page_number(update);
        }
    }

    total_middle_page_number
}

fn cal_second_part_answer(
    page_ordering_rules: &HashMap<i32, Vec<i32>>,
    updates: &Vec<Vec<i32>>,
) -> i32 {
    let mut total_middle_page_number = 0;

    for update in updates {
        if !is_in_the_right_order(page_ordering_rules, update) {
            total_middle_page_number +=
                find_middle_page_number(&arrange_update(page_ordering_rules, update));
        }
    }

    total_middle_page_number
}

fn is_in_the_right_order(page_ordering_rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    for (i, u) in update.iter().enumerate() {
        if i == update.len() - 1 {
            return true;
        }

        match page_ordering_rules.get(u) {
            Some(rules) => {
                if !rules.contains(&update[i + 1]) {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }

    true
}

fn arrange_update(page_ordering_rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut arranged_update = update.clone();

    for (i, u) in update.iter().enumerate() {
        if i == update.len() - 1 {
            break;
        }

        if let Some(rules) = page_ordering_rules.get(u) {
            // when rule is empty means this page is the last
            if rules.is_empty() {
                arranged_update.swap(i, update.len() - 1);
                return arrange_update(page_ordering_rules, &arranged_update);
            }

            if rules.contains(&update[i + 1]) {
                continue;
            }
        }

        arranged_update.swap(i, i + 1);
        return arrange_update(page_ordering_rules, &arranged_update);
    }

    arranged_update
}

fn find_middle_page_number(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_the_right_order() {
        let mut page_ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        page_ordering_rules.insert(75, vec![53, 47, 61, 53, 29]);
        page_ordering_rules.insert(47, vec![53, 13, 61, 29]);
        page_ordering_rules.insert(61, vec![13, 53, 29]);
        page_ordering_rules.insert(53, vec![29, 13]);
        page_ordering_rules.insert(29, vec![13]);

        let updates = vec![75, 47, 61, 53, 29];
        assert_eq!(is_in_the_right_order(&page_ordering_rules, &updates), true);
    }

    #[test]
    fn test_find_middle_page_number() {
        assert_eq!(find_middle_page_number(&vec![75, 47, 61, 53, 29]), 61);
        assert_eq!(find_middle_page_number(&vec![97, 61, 53, 29, 13]), 53);
        assert_eq!(find_middle_page_number(&vec![75, 29, 13]), 29);
    }

    #[test]
    fn test_arrange_update() {
        let mut page_ordering_rules: HashMap<i32, Vec<i32>> = HashMap::new();
        page_ordering_rules.insert(97, vec![13, 61, 47, 29, 53, 75]);
        page_ordering_rules.insert(75, vec![53, 47, 61, 53, 29]);
        page_ordering_rules.insert(47, vec![53, 13, 61, 29]);
        page_ordering_rules.insert(61, vec![13, 53, 29]);
        page_ordering_rules.insert(53, vec![29, 13]);
        page_ordering_rules.insert(29, vec![13]);
        page_ordering_rules.insert(13, vec![]);

        assert_eq!(
            arrange_update(&page_ordering_rules, &vec![75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            arrange_update(&page_ordering_rules, &vec![97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }
}
