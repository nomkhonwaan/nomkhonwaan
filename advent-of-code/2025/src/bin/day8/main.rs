use advent_of_code_2025::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_lines(&args[1]).unwrap();
    let junction_boxes: Vec<JunctionBox> = parse(&input);

    println!(
        "First part answer: {}",
        cal_first_part_answer(&junction_boxes, 1000)
    );
    println!(
        "Second part answer: {}",
        cal_second_part_answer(&junction_boxes).unwrap()
    );
}

fn parse(input: &[String]) -> Vec<JunctionBox> {
    input
        .iter()
        .enumerate()
        .map(|(id, coordinate)| JunctionBox::new(id, coordinate))
        .collect()
}

fn cal_first_part_answer(junction_boxes: &[JunctionBox], target_connections: usize) -> usize {
    let mut pairs: Vec<(usize, usize, f64)> = vec![];
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            pairs.push((
                junction_boxes[i].id,
                junction_boxes[j].id,
                junction_boxes[i].cal_distance(&junction_boxes[j]),
            ));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut dsu = DisjointSetUnion::new(junction_boxes.len());
    for k in 0..target_connections {
        let (a, b, _) = pairs[k];
        dsu.union(a, b);
    }

    let mut circuit_sizes = vec![];
    for i in 0..junction_boxes.len() {
        if dsu.parent[i] == i {
            circuit_sizes.push(dsu.size[i]);
        }
    }

    circuit_sizes.sort_by(|a, b| b.cmp(a));
    circuit_sizes.iter().take(3).product()
}

fn cal_second_part_answer(junction_boxes: &[JunctionBox]) -> Result<i64, ()> {
    let mut pairs: Vec<(usize, usize, f64)> = vec![];
    for i in 0..junction_boxes.len() {
        for j in i + 1..junction_boxes.len() {
            pairs.push((
                junction_boxes[i].id,
                junction_boxes[j].id,
                junction_boxes[i].cal_distance(&junction_boxes[j]),
            ));
        }
    }
    pairs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut dsu = DisjointSetUnion::new(junction_boxes.len());
    let mut num_components = junction_boxes.len();
    for (a, b, _) in pairs.into_iter() {
        if dsu.union(a, b) {
            num_components -= 1;

            if num_components == 1 {
                return Ok(junction_boxes[a].x * junction_boxes[b].x);
            }
        }
    }

    Err(())
}

#[derive(Debug, Default, PartialEq)]
struct JunctionBox {
    id: usize,
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(id: usize, coordiate: &str) -> Self {
        let coordinate: Vec<&str> = coordiate.split(",").collect();
        Self {
            id,
            x: coordinate[0].parse().unwrap(),
            y: coordinate[1].parse().unwrap(),
            z: coordinate[2].parse().unwrap(),
        }
    }

    fn cal_distance(&self, b: &JunctionBox) -> f64 {
        let x = (self.x - b.x).pow(2);
        let y = (self.y - b.y).pow(2);
        let z = (self.z - b.z).pow(2);
        ((x + y + z) as f64).sqrt()
    }
}

#[derive(Debug, Default)]
struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.find(self.parent[x]);
            self.parent[x]
        }
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            JunctionBox::new(0, "984,92,344"),
            JunctionBox {
                id: 0,
                x: 984,
                y: 92,
                z: 344
            }
        )
    }

    #[test]
    fn test_cal_first_part_answer() {
        let input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];
        let junction_boxes = parse(&input);

        assert_eq!(cal_first_part_answer(&junction_boxes, 10), 40);
    }

    #[test]
    fn test_cal_second_part_answer() {
        let input = vec![
            "162,817,812".to_string(),
            "57,618,57".to_string(),
            "906,360,560".to_string(),
            "592,479,940".to_string(),
            "352,342,300".to_string(),
            "466,668,158".to_string(),
            "542,29,236".to_string(),
            "431,825,988".to_string(),
            "739,650,466".to_string(),
            "52,470,668".to_string(),
            "216,146,977".to_string(),
            "819,987,18".to_string(),
            "117,168,530".to_string(),
            "805,96,715".to_string(),
            "346,949,466".to_string(),
            "970,615,88".to_string(),
            "941,993,340".to_string(),
            "862,61,35".to_string(),
            "984,92,344".to_string(),
            "425,690,689".to_string(),
        ];
        let junction_boxes = parse(&input);

        assert_eq!(cal_second_part_answer(&junction_boxes).unwrap(), 25272);
    }
}
