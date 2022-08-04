use std::{env, fs, io, path};
use std::collections::HashMap;
use std::io::BufRead;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut list_of_rectangles: Vec<Rectangle> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let rectangle = Rectangle::from_string(line);
                list_of_rectangles.push(rectangle);
            }
        }
    }

    let collided_points: HashMap<(i32, i32), i32> = list_of_rectangles
        .iter()
        .flat_map(|rectangle| rectangle.to_points())
        .fold(HashMap::new(), |mut result, pair| {
            // insert pair to the hash map and increase count by 1
            result.insert(pair, match result.get(&pair) {
                Some(count) => count + 1,
                _ => 1,
            });
            result
        })
        .into_iter()
        .filter(|(_, v)| *v > 1) // to filter only points are collided more than 1
        .collect();

    println!("first part answer is: {}", collided_points.len());
}

/// Contain each rectangle data.
#[derive(Default)]
struct Rectangle {
    // id: String,
    left_edge: i32,
    top_edge: i32,
    wide: i32,
    tall: i32,
}

impl Rectangle {
    fn from_string(s: String) -> Self {
        let re = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
        let captures = re.captures(&s).unwrap();

        Rectangle {
            // id: captures.get(1).map(|m| m.as_str()).unwrap().to_string(),
            left_edge: captures.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            top_edge: captures.get(3).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            wide: captures.get(4).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            tall: captures.get(5).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            ..Default::default()
        }
    }

    fn to_points(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = vec![];
        for i in self.left_edge..self.left_edge + self.wide {
            for j in self.top_edge..self.top_edge + self.tall {
                points.push((i, j));
            }
        }
        points
    }
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}