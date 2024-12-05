use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut reports: Vec<Report> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                reports.push(Report::from_str(&line));
            }
        }
    }

    let first_part_answer = reports.iter().filter(|r| r.is_safe()).count();
    let second_part_answer = reports.iter().filter(|r| r.also_safe()).count();

    println!("First part answer is: {}", first_part_answer);
    println!("Second part answer is: {}", second_part_answer);
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Debug, Default)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn from_str(s: &str) -> Self {
        let levels: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        is_safe(&self.levels)
    }

    fn also_safe(&self) -> bool {
        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();

            levels.remove(i);         

            if is_safe(&levels) {
                return true;
            }
        }

        false
    }
}

fn is_safe(levels: &Vec<i32>) -> bool {
    // The levels are either all increasing or all decreasing.
    if !is_increasing(&levels) && !is_decreasing(&levels){
        return false;
    }

    for (i, level) in levels.iter().enumerate() {
        if i > 0 {
            let diff = (level - levels[i - 1]).abs();

            if diff < 1 || diff > 3 {
                // Any two adjacent levels differ by at least one and at most three.
                return false;
            }
        }
    }

    true
}

fn is_increasing(levels: &Vec<i32>) -> bool {
    for i in 1..levels.len() {
        if levels[i] <= levels[i - 1] {
            return false;
        }
    }
    true
}

fn is_decreasing(levels: &Vec<i32>) -> bool {
    for i in 1..levels.len() {
        if levels[i] >= levels[i - 1] {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_from_str() {
        let report = Report::from_str("1 2 3 4 5");
        assert_eq!(report.levels, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_report_is_safe() {
        let report = Report::from_str("1 3 6 7 9");
        assert_eq!(report.is_safe(), true);

        let report = Report::from_str("7 6 4 2 1");
        assert_eq!(report.is_safe(), true);

        let report = Report::from_str("8 6 4 4 1");
        assert_eq!(report.is_safe(), false);

        let report = Report::from_str("1 3 2 4 5");
        assert_eq!(report.is_safe(), false);
    }

    #[test]
    fn test_report_also_safe() {
        let report = Report::from_str("77 78 80 81 80 82");
        assert_eq!(report.also_safe(), true);

        let report = Report::from_str("1 1 2 3 4");
        assert_eq!(report.also_safe(), true);

        let report = Report::from_str("2 5 4 3 2 ");
        assert_eq!(report.also_safe(), true);
    }
}
