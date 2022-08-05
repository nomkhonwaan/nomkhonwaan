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

    let counted_points: HashMap<(i32, i32), i32> = list_of_rectangles
        .clone()
        .iter()
        // at this point, the rectangle will become vector of points
        //
        // [(x0,y0), (x1,y1), ..., (xn, yn)]
        .flat_map(|rectangle| rectangle.to_points())
        // at this point, the vector of points will become a hashmap like this
        //
        // [
        //   (x0,y0): 1,
        //   (x1,y1): 2,
        //   ...
        //   (xn,yn): 1,
        // ]
        .fold(HashMap::new(), |mut result, pair| {
            // insert pair to the hash map and increase count by 1
            result.insert(pair, match result.get(&pair) {
                Some(count) => count + 1,
                _ => 1,
            });
            result
        });

    let collided_points: HashMap<(i32, i32), i32> = counted_points
        .clone()
        .into_iter()
        .filter(|(_, v)| *v > 1) // to filter only points are collided more than 1
        .collect();
    let non_collided_points: Vec<(i32, i32)> = counted_points
        .clone()
        .into_iter()
        .filter(|(_, v)| *v == 1) // to filter only points are not counted more than 1
        // at this point, the hashmap will become a vector of points
        //
        // [(x0,y0), (x1,y1), ..., (xn,yx)]
        .map(|(k, _)| k)
        .collect();

    println!("first part answer is: {}", collided_points.len());

    for rectangle in list_of_rectangles.into_iter() {
        let yes = rectangle.to_points()
            .iter()
            .all(|point| {
                non_collided_points.contains(point)
            });
        if yes {
            println!("second part answer is: {}", &rectangle.id);
            break;
        }
    }
}

/// Contain each rectangle data.
#[derive(Clone)]
struct Rectangle {
    id: String,
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
            id: captures.get(1).map(|m| m.as_str().to_string()).unwrap(),
            left_edge: captures.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            top_edge: captures.get(3).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            wide: captures.get(4).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            tall: captures.get(5).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
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