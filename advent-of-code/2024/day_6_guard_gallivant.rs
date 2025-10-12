use std::{
    collections::HashSet,
    env, fs,
    io::{self, BufRead},
    path,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut obstructions: Vec<(usize, usize)> = vec![];
    let (mut max_i, mut max_j): (usize, usize) = (0, 1);
    let mut starting_position: (usize, usize) = (0, 0);
    let mut starting_direction: char = ' ';

    if let Ok(lines) = read_lines(input) {
        for (i, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (j, s) in line.chars().enumerate() {
                    match s {
                        '#' => {
                            obstructions.push((i, j));
                        }
                        '^' | '>' | 'v' | '<' => {
                            starting_position = (i, j);
                            starting_direction = s;
                        }
                        _ => {}
                    }
                    max_j = j + 1;
                }
            }
            max_i += 1;
        }
    }

    println!(
        "First part answer is: {}",
        cal_first_part_answer(
            &obstructions,
            &starting_position,
            &starting_direction,
            max_i,
            max_j
        )
    );

    println!(
        "Second part answer is: {}",
        cal_second_part_answer(
            &obstructions,
            &starting_position,
            &starting_direction,
            max_i,
            max_j
        )
        .await
    );
}

fn cal_first_part_answer(
    obstructions: &Vec<(usize, usize)>,
    starting_position: &(usize, usize),
    starting_direction: &char,
    max_i: usize,
    max_j: usize,
) -> i32 {
    let mut current_position = *starting_position;
    let mut current_direction = *starting_direction;
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::from([current_position]);

    let direction_moves: Vec<char> = vec!['^', '>', 'v', '<'];
    let direction_deltas: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    loop {
        let current_direction_index = direction_moves
            .iter()
            .position(|&d| d == current_direction)
            .unwrap();

        let next_position = (
            (current_position.0 as i32 + direction_deltas[current_direction_index].0) as usize,
            (current_position.1 as i32 + direction_deltas[current_direction_index].1) as usize,
        );

        if obstructions.contains(&next_position) {
            // Turn right
            current_direction =
                direction_moves[(current_direction_index + 1) % direction_moves.len()];
            continue;
        } else if next_position.0 >= max_i || next_position.1 >= max_j {
            break;
        } else {
            current_position = next_position;
            visited_positions.insert(current_position);
        }
    }

    visited_positions.len() as i32
}

async fn cal_second_part_answer(
    obstructions: &Vec<(usize, usize)>,
    starting_position: &(usize, usize),
    starting_direction: &char,
    max_i: usize,
    max_j: usize,
) -> i32 {
    // Convert to HashSet for faster lookups
    let obstruction_set: HashSet<(usize, usize)> = obstructions.iter().cloned().collect();

    // Collect all positions to test
    let mut test_positions = Vec::new();
    for i in 0..max_i {
        for j in 0..max_j {
            // Skip if this position is the starting position or already an obstruction
            if (i, j) == *starting_position || obstruction_set.contains(&(i, j)) {
                continue;
            }
            test_positions.push((i, j));
        }
    }

    // Create async tasks for each test position
    let mut tasks = Vec::new();
    for test_pos in test_positions {
        let obstruction_set_clone = obstruction_set.clone();
        let starting_pos = *starting_position;
        let starting_dir = *starting_direction;

        let task = tokio::spawn(async move {
            if test_obstruction_creates_loop(
                &obstruction_set_clone,
                test_pos,
                starting_pos,
                starting_dir,
                max_i,
                max_j,
            ) {
                Some(test_pos)
            } else {
                None
            }
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete and collect results
    let mut valid_positions = HashSet::new();
    for task in tasks {
        if let Ok(Some(position)) = task.await {
            valid_positions.insert(position);
        }
    }

    valid_positions.len() as i32
}

// Helper function to test if adding an obstruction at (test_i, test_j) creates a loop
fn test_obstruction_creates_loop(
    obstructions: &HashSet<(usize, usize)>,
    test_position: (usize, usize),
    starting_position: (usize, usize),
    starting_direction: char,
    max_i: usize,
    max_j: usize,
) -> bool {
    let direction_moves: Vec<char> = vec!['^', '>', 'v', '<'];
    let direction_deltas: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Create a new set of obstructions including the test position
    let mut existing_obstructions = obstructions.clone();
    existing_obstructions.insert(test_position);

    let mut current_position = starting_position;
    let mut current_direction = starting_direction;
    let mut state_history: HashSet<(usize, usize, char)> = HashSet::new();

    // Insert initial state
    state_history.insert((current_position.0, current_position.1, current_direction));

    loop {
        let current_direction_index = direction_moves
            .iter()
            .position(|&d| d == current_direction)
            .unwrap();

        let next_position = (
            (current_position.0 as i32 + direction_deltas[current_direction_index].0) as usize,
            (current_position.1 as i32 + direction_deltas[current_direction_index].1) as usize,
        );

        if existing_obstructions.contains(&next_position) {
            // Turn right
            current_direction =
                direction_moves[(current_direction_index + 1) % direction_moves.len()];
            continue;
        } else if next_position.0 >= max_i || next_position.1 >= max_j {
            break;
        } else {
            current_position = next_position;
        }

        // Check if we've seen this (position, direction) state before
        let state = (current_position.0, current_position.1, current_direction);
        if state_history.contains(&state) {
            return true; // Loop detected
        }
        state_history.insert(state);
    }

    false // No loop detected
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
