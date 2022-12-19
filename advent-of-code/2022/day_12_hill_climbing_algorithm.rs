use std::collections::{HashSet, VecDeque};
use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut heightmap: Vec<Vec<char>> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let line = line.chars().collect();
                heightmap.push(line);
            }
        }
    }

    let s = find_first(&heightmap, 'S').unwrap();
    let e = find_first(&heightmap, 'E').unwrap();
    heightmap[s.0][s.1] = 'a';
    heightmap[e.0][e.1] = 'z';

    println!("first part answer is: {}", climb(&heightmap, vec![s], e));
    println!("second part answer is: {}", climb(&heightmap, find_all(&heightmap, 'a'), e));
}

fn climb(heightmap: &Vec<Vec<char>>, starts: Vec<(usize, usize)>, goal: (usize, usize)) -> u32 {
    let max_m = heightmap.len();
    let max_n = heightmap[0].len();
    let mut shortest_path = u32::MAX;

    for s in starts {
        let mut queue: VecDeque<(usize, usize, u32)> = VecDeque::from(vec![(s.0, s.1, 0)]);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();

            if (p.0, p.1) == goal {
                if p.2 < shortest_path {
                    shortest_path = p.2;
                }
            }

            for adj in find_adjacents(&heightmap, &(p.0, p.1), max_m, max_n) {
                if visited.insert(adj) {
                    queue.push_back((adj.0, adj.1, p.2 + 1));
                }
            }
        }
    }

    shortest_path
}

fn find_first(heightmap: &Vec<Vec<char>>, expected: char) -> Option<(usize, usize)> {
    for (i, row) in heightmap.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == expected {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_all(heightmap: &Vec<Vec<char>>, expected: char) -> Vec<(usize, usize)> {
    let mut points = vec![];
    for (i, row) in heightmap.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == expected {
                points.push((i, j));
            }
        }
    }
    points
}

fn find_adjacents(
    heightmap: &Vec<Vec<char>>,
    p: &(usize, usize),
    max_m: usize,
    max_n: usize,
) -> Vec<(usize, usize)> {
    let current_height = heightmap[p.0][p.1];
    let mut points = vec![];

    if p.0 > 0 {
        points.push((p.0 - 1, p.1));
    }
    if p.0 + 1 < max_m {
        points.push((p.0 + 1, p.1));
    }
    if p.1 > 0 {
        points.push((p.0, p.1 - 1));
    }
    if p.1 + 1 < max_n {
        points.push((p.0, p.1 + 1));
    }

    points
        .into_iter()
        .filter(|adj| {
            let height = heightmap[adj.0][adj.1];
            height as i32 - current_height as i32 <= 1
        })
        .collect()
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first() {
        let heightmap: Vec<Vec<char>> = vec![
            vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
        ];

        assert_eq!(Some((0, 0)), find_first(&heightmap, 'S'));
        assert_eq!(Some((2, 5)), find_first(&heightmap, 'E'));
    }

    #[test]
    fn test_find_all() {
        let heightmap: Vec<Vec<char>> = vec![
            vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
        ];

        assert_eq!(
            vec![(0, 1), (1, 0), (2, 0), (3, 0), (4, 0)],
            find_all(&heightmap, 'a')
        );
    }

    #[test]
    fn test_find_adjacents() {
        let heightmap: Vec<Vec<char>> = vec![
            vec!['a', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'z', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
        ];
        let max_m = heightmap.len();
        let max_n = heightmap[0].len();

        assert_eq!(
            vec![(3, 0), (4, 1)],
            find_adjacents(&heightmap, &(4, 0), max_m, max_n)
        );
        assert_eq!(
            vec![(3, 7), (4, 6)],
            find_adjacents(&heightmap, &(4, 7), max_m, max_n)
        );
        assert_eq!(
            vec![(1, 0), (0, 1)],
            find_adjacents(&heightmap, &(0, 0), max_m, max_n)
        );
        assert_eq!(
            vec![(1, 2), (3, 2), (2, 1)],
            find_adjacents(&heightmap, &(2, 2), max_m, max_n)
        );
        assert_eq!(
            vec![(0, 3), (2, 3), (1, 2)],
            find_adjacents(&heightmap, &(1, 3), max_m, max_n)
        );
    }
}
