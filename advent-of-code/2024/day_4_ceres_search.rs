use std::{
    env, fs,
    io::{self, BufRead},
    path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut matrix: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                matrix.push(line.chars().collect::<Vec<char>>());
            }
        }
    }

    let xmas = find_all_xmas(&matrix);
    let x_mas = find_all_x_mas(&matrix);

    // the same word can be counted twice, so divide by 2
    let first_part_answer = xmas.iter().filter(|&x| match_xmas(x)).count() / 2;
    let second_part_answer = x_mas
        .iter()
        .filter(|&x| x.iter().filter(|&y| match_mas(y)).count() == x.len())
        .count();
    println!("First part answer is: {}", first_part_answer);
    println!("Second part answer is: {}", second_part_answer);
}

fn find_all_xmas(matrix: &Vec<Vec<char>>) -> Vec<String> {
    let mut xmas: Vec<String> = Vec::new();
    let directions: [(isize, isize); 8] = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // down-right
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, 0),  // up
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            for &(di, dj) in &directions {
                if i as isize + 3 * di < matrix.len() as isize
                    && i as isize + 3 * di >= 0
                    && j as isize + 3 * dj < matrix[i].len() as isize
                    && j as isize + 3 * dj >= 0
                {
                    let mut sequence: Vec<char> = Vec::new();
                    for k in 0..4 {
                        sequence.push(
                            matrix[(i as isize + k * di) as usize][(j as isize + k * dj) as usize],
                        );
                    }
                    xmas.push(sequence.into_iter().collect());
                }
            }
        }
    }

    xmas
}

fn find_all_x_mas(matrix: &Vec<Vec<char>>) -> Vec<Vec<String>> {
    let mut x_mas: Vec<Vec<String>> = Vec::new();
    let directions: [[(isize, isize); 3]; 2] = [
        [
            (-1, -1), /* up-left */
            (0, 0),   /* middle */
            (1, 1),   /* down-right */
        ],
        [
            (-1, 1), /* up-right */
            (0, 0),  /* middle */
            (1, -1), /* down-left */
        ],
    ];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut t_x_mas: Vec<String> = Vec::new();

            for &direction in &directions {
                let mut sequence: Vec<char> = Vec::new();

                for &(di, dj) in &direction {
                    if i as isize + di < matrix.len() as isize
                        && i as isize + di >= 0
                        && j as isize + dj < matrix[i].len() as isize
                        && j as isize + dj >= 0
                    {
                        sequence
                            .push(matrix[(i as isize + di) as usize][(j as isize + dj) as usize]);
                    }
                }

                t_x_mas.push(sequence.into_iter().collect());
            }

            x_mas.push(t_x_mas);
        }
    }

    x_mas
}

fn match_xmas(s: &str) -> bool {
    // check the string `s` matching either XMAS or SAMX
    s == "XMAS" || s == "SAMX"
}

fn match_mas(s: &str) -> bool {
    // check the string `s` matching either MAS or SAM
    s == "MAS" || s == "SAM"
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
