use advent_of_code_2025::read_lines;
use std::{cmp, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_lines(&args[1]).unwrap();
    let vertices = parse(&input);

    println!("First part answer: {}", cal_first_part_answer(&vertices));
    println!("Second part answer: {}", cal_second_part_answer(&vertices));
}

fn parse(input: &[String]) -> Vec<(i64, i64)> {
    input
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect()
}

fn cal_rectangle_area(a: (i64, i64), b: (i64, i64)) -> i64 {
    let width = (a.0 - b.0).abs() + 1;
    let height = (a.1 - b.1).abs() + 1;
    width * height
}

fn cal_first_part_answer(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    let mut max_area = 0i64;

    for i in 0..n {
        for j in i..n {
            let area = cal_rectangle_area(vertices[i], vertices[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn cal_second_part_answer(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    let mut candidates = vec![];

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[j];

            if x1 == x2 || y1 == y2 {
                continue;
            }

            let area = cal_rectangle_area(vertices[i], vertices[j]);
            let x_min = cmp::min(x1, x2);
            let x_max = cmp::max(x1, x2);
            let y_min = cmp::min(y1, y2);
            let y_max = cmp::max(y1, y2);

            candidates.push((area, x_min, x_max, y_min, y_max));
        }
    }

    candidates.sort_by(|a, b| b.0.cmp(&a.0));

    for (area, x_min, x_max, y_min, y_max) in candidates {
        if is_rectangle_valid(x_min, x_max, y_min, y_max, vertices) {
            return area;
        }
    }

    0
}

fn is_rectangle_valid(x_min: i64, x_max: i64, y_min: i64, y_max: i64, vertices: &[(i64, i64)]) -> bool {
    if has_vertex_inside(x_min, x_max, y_min, y_max, vertices) {
        return false;
    }

    let n = vertices.len();
    for i in 0..n {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % n];

        if edge_cuts_rectangle(x_min, x_max, y_min, y_max, p1, p2) {
            return false;
        }
    }

    true
}

fn has_vertex_inside(x_min: i64, x_max: i64, y_min: i64, y_max: i64, vertices: &[(i64, i64)]) -> bool {
    vertices
        .iter()
        .any(|&(x, y)| x > x_min && x < x_max && y > y_min && y < y_max)
}

fn edge_cuts_rectangle(
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    p1: (i64, i64),
    p2: (i64, i64),
) -> bool {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    if x1 == x2 {
        let edge_y_min = std::cmp::min(y1, y2);
        let edge_y_max = std::cmp::max(y1, y2);
        x1 > x_min && x1 < x_max && !(edge_y_max <= y_min || edge_y_min >= y_max)
    } else {
        let edge_x_min = std::cmp::min(x1, x2);
        let edge_x_max = std::cmp::max(x1, x2);
        y1 > y_min && y1 < y_max && !(edge_x_max <= x_min || edge_x_min >= x_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = &vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];

        assert_eq!(
            parse(input),
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3)
            ]
        )
    }

    #[test]
    fn test_cal_rectangle_area() {
        assert_eq!(cal_rectangle_area((0, 0), (2, 2)), 9)
    }

    #[test]
    fn test_cal_first_part_answer() {
        let input = &vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        let vertices = parse(&input);

        assert_eq!(cal_first_part_answer(&vertices), 50)
    }

    #[test]
    fn test_cal_second_part_answer() {
        let input = &vec![
            "7,1".to_string(),
            "11,1".to_string(),
            "11,7".to_string(),
            "9,7".to_string(),
            "9,5".to_string(),
            "2,5".to_string(),
            "2,3".to_string(),
            "7,3".to_string(),
        ];
        let vertices = parse(&input);

        assert_eq!(cal_second_part_answer(&vertices), 24)
    }
}
