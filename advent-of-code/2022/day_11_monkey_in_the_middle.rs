use regex::Regex;
use std::{collections::VecDeque, env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut monkeys: Vec<Monkey> = vec![];

    if let Ok(lines) = read_lines(input) {
        let mut buf = String::new();
        for line in lines {
            if let Ok(line) = line {
                buf.push_str(&format!("{}\n", line));

                if line.is_empty() {
                    monkeys.push(buf.trim_end().parse().unwrap());
                    buf = String::new();
                }
            }
        }

        // for the latest monkey,
        // otherwise we will need to add an extra newline
        // at the end of the input file
        monkeys.push(buf.trim_end().parse().unwrap());
    }

    // To avoid using unsafe pattern,
    // let creates another playbook which will
    // mutating number of items are holding by the monkey.
    let mut playbook: Vec<VecDeque<u64>> =
        monkeys.iter().map(|m| m.starting_items.clone()).collect();

    for _ in 0..20 {
        for (i, m) in monkeys.iter_mut().enumerate() {
            while let Some(worry_level) = playbook[i].pop_front() {
                let worry_level = m.inspects(worry_level) / 3;

                match worry_level % m.divisible_by {
                    0 => {
                        playbook[m.if_true_throw_to_monkey].push_back(worry_level);
                    }
                    _ => {
                        playbook[m.if_false_throw_to_monkey].push_back(worry_level);
                    }
                }
            }
        }
    }

    let inspected_times = list_of_inspected_items_times(&monkeys);
    println!(
        "first part answer is: {}",
        inspected_times.iter().take(2).product::<u64>()
    );

    // shadow declaration for finding the second part answer
    // also resetting the number of inspected items times
    let mut playbook: Vec<VecDeque<u64>> = monkeys
        .iter_mut()
        .map(|mut m| {
            m.inspected_items_times = 0;
            m.starting_items.clone()
        })
        .collect();
    let common_denominator: u64 = monkeys.iter().map(|m| m.divisible_by).product();

    for _ in 0..10_000 {
        for (i, m) in monkeys.iter_mut().enumerate() {
            while let Some(worry_level) = playbook[i].pop_front() {
                let worry_level = m.inspects(worry_level) % common_denominator;

                match worry_level % m.divisible_by {
                    0 => {
                        playbook[m.if_true_throw_to_monkey].push_back(worry_level);
                    }
                    _ => {
                        playbook[m.if_false_throw_to_monkey].push_back(worry_level);
                    }
                }
            }
        }
    }

    let inspected_times = list_of_inspected_items_times(&monkeys);
    println!(
        "second part answer is: {:?}",
        inspected_times.iter().take(2).product::<u64>()
    );
}

fn list_of_inspected_items_times(monkeys: &Vec<Monkey>) -> Vec<u64> {
    let mut list_of_inspected_items_times: Vec<u64> =
        monkeys.iter().map(|m| m.inspected_items_times).collect();
    list_of_inspected_items_times.sort_by(|m, n| n.cmp(m)); // sort by descending
    list_of_inspected_items_times
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

trait Operation {
    fn operate(&self, n: u64) -> u64;
}

impl<F> Operation for F
where
    F: Fn(u64) -> u64,
{
    fn operate(&self, n: u64) -> u64 {
        self(n)
    }
}

/// Accept static value m, return m add by n.
fn add_by(m: u64) -> Box<dyn Operation> {
    Box::new(move |n: u64| n + m)
}

/// Accept static value m, return m multiply by n.
fn multiply_by(m: u64) -> Box<dyn Operation> {
    Box::new(move |n: u64| n * m)
}

/// Return double n.
fn double() -> Box<dyn Operation> {
    Box::new(|n: u64| n + n)
}

/// Return square n.
fn squre() -> Box<dyn Operation> {
    Box::new(|n: u64| n * n)
}

struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Box<dyn Operation>,
    divisible_by: u64,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
    inspected_items_times: u64,
}

impl Monkey {
    fn inspects(&mut self, worry_level: u64) -> u64 {
        self.inspected_items_times += 1;
        self.operation.operate(worry_level)
    }
}

impl std::str::FromStr for Monkey {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Monkey {
            starting_items: parse_starting_items(s),
            operation: parse_operation(s),
            divisible_by: parse_divisible_by(s),
            if_true_throw_to_monkey: parse_if_true_throw_to_monkey(s),
            if_false_throw_to_monkey: parse_if_false_throw_to_monkey(s),
            inspected_items_times: 0u64,
        })
    }
}

fn parse_starting_items(s: &str) -> VecDeque<u64> {
    let re = Regex::new(r"Starting items: (.+)\n").unwrap();
    let cap = re.captures(s).unwrap();
    let cap: Vec<&str> = cap.get(1).unwrap().as_str().split(",").collect();
    let mut starting_items = vec![];
    for v in cap.into_iter() {
        starting_items.push(v.trim().parse().unwrap());
    }
    VecDeque::from(starting_items)
}

/// To keep it easy since this is not the majority of the problem,
/// I will use regular expression to match each type of the operation
/// and return a static pre-defined operation function rather than
/// construction from the given variable since it will very too complicated in Rust!
fn parse_operation(s: &str) -> Box<dyn Operation> {
    let re = Regex::new(r"Operation: new = (.+)\n").unwrap();
    let cap = re.captures(s).unwrap().get(1).unwrap().as_str();
    let re_add_by = Regex::new(r"old \+ (\d+)").unwrap();
    let re_multiply_by = Regex::new(r"old \* (\d+)").unwrap();
    let re_double = Regex::new(r"old \+ old").unwrap();
    let re_square = Regex::new(r"old \* old").unwrap();

    if re_add_by.is_match(cap) {
        return add_by(
            re_add_by
                .captures(cap)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
        );
    } else if re_multiply_by.is_match(cap) {
        return multiply_by(
            re_multiply_by
                .captures(cap)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
        );
    } else if re_double.is_match(cap) {
        return double();
    } else if re_square.is_match(cap) {
        return squre();
    }

    panic!("invalid operation or not supported");
}

fn parse_divisible_by(s: &str) -> u64 {
    let re = Regex::new(r"Test: divisible by (.+)\n").unwrap();
    re.captures(s)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn parse_if_true_throw_to_monkey(s: &str) -> usize {
    let re = Regex::new(r"If true: throw to monkey (.+)\n").unwrap();
    re.captures(s)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn parse_if_false_throw_to_monkey(s: &str) -> usize {
    let re = Regex::new(r"If false: throw to monkey (.+)$").unwrap();
    re.captures(s)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_starting_items() {
        let s = "Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";

        assert_eq!(
            VecDeque::from(vec![54, 65, 75, 74]),
            parse_starting_items(s)
        );
    }

    #[test]
    fn test_parse_operation() {
        let s = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        assert_eq!(76, parse_operation(s).operate(4));
    }

    #[test]
    fn test_parse_divisible_by() {
        let s = "Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3";

        assert_eq!(13, parse_divisible_by(s));
    }

    #[test]
    fn test_parse_if_true_throw_to_monkey() {
        let s = "Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(0, parse_if_true_throw_to_monkey(s));
    }

    #[test]
    fn test_parse_if_false_throw_to_monkey() {
        let s = "Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        assert_eq!(1, parse_if_false_throw_to_monkey(s));
    }
}
