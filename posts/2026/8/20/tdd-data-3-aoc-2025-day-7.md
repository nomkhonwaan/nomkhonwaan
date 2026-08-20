---
title: "TDD Kata 3 - Advent of Code 2025 Day 7: Laboratories"
publish_date: 2026-08-20
tags: ['tdd', 'kata', 'rust', 'advent-of-code']
---

กลับมาพบกันอีกครั้งกับ TDD Kata ครั้งนี้เราจะมาลองใช้ TDD เพื่อแก้โจทย์ [Advent of Code 2025 Day 7: Laboratories](https://adventofcode.com/2025/day/7) ด้วยภาษา Rust กัน

โจทย์จำลองสถานการณ์ที่เราอยู่ในห้องทดลองเกี่ยวกับการเทเลพอร์ต และเจอข้อผิดพลาด `0H-N0` ที่เกี่ยวข้องกับ tachyon manifold เราจึงต้องวิเคราะห์แผนผังของ manifold เพื่อหาว่าเกิดการแยกของลำแสง (beam split) ขึ้นกี่ครั้ง

## สารบัญ

- [TDD Kata 1 - String Calculator](/2015/6/1/tdd-kata-1-string-calculator)
- [TDD Kata 2 - Bowling Game](/2016/1/28/tdd-kata-2-the-bowling-game)
- [TDD Kata 3 - Advent of Code 2025 Day 7: Laboratories](/2026/8/20/tdd-data-3-aoc-2025-day-7)

## TL;DR

[GitHub]((https://github.com/nomkhonwaan/nomkhonwaan/blob/main/advent-of-code/2025/src/bin/day7))

---

## กติกา

แผนผังของ tachyon manifold เป็นตารางกริดที่มีสัญลักษณ์ดังนี้:

- `S` - จุดเริ่มต้นของลำแสง (tachyon beam) ลำแสงจะเคลื่อนที่ลงด้านล่างเสมอ
- `.` - ช่องว่าง ลำแสงสามารถผ่านได้
- `^` - ตัวแยกลำแสง (splitter) เมื่อลำแสงชนจะหยุดและแยกออกเป็นสองลำแสงใหม่ โดยลำแสงใหม่จะเคลื่อนที่ต่อจากด้านซ้ายและด้านขวาของ splitter

ตัวอย่างแผนผัง:

```
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
```

เมื่อลำแสงเคลื่อนที่ลงมาเรื่อย ๆ จะเกิดการแยกตัวออกไปเรื่อย ๆ จนกระทั่งลำแสงทั้งหมดออกจากแผนผังหรือเจอ splitter:

```
.......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
|.|.|.|.|.|||.|
```

ในตัวอย่างนี้เกิดการแยกลำแสงทั้งหมด 21 ครั้ง

---

## เตรียมพร้อม

โจทย์นี้เราจะใช้โปรเจกต์ Rust ที่มีอยู่แล้วใน `advent-of-code/2025` โดยเพิ่ม binary ใหม่สำหรับ day 7

เริ่มต้นด้วยการเพิ่ม `[[bin]]` ใน `Cargo.toml`:

```toml
[[bin]]
name = "day7"
path = "src/bin/day7/main.rs"
```

จากนั้นสร้างโครงสร้างไฟล์:

```bash
mkdir -p advent-of-code/2025/src/bin/day7
touch advent-of-code/2025/src/bin/day7/main.rs
```

ในโปรเจกต์นี้มี library utilities ไว้ให้แล้ว เช่น `advent_of_code_2025::read_lines` สำหรับอ่านไฟล์อินพุต

ก่อนอื่นมาเขียนโครงหลักของโปรแกรมที่ใช้อ่านอินพุตจากไฟล์ตามสไตล์ AOC:

```rust
// src/bin/day7/main.rs

use advent_of_code_2025::read_lines;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_lines(&args[1]).unwrap();
    let grid = parse_grid(&input);

    println!("First part answer: {}", cal_first_part_answer(&grid));
    println!("Second part answer: {}", cal_second_part_answer(&grid));
}

fn parse_grid(input: &[String]) -> Vec<Vec<u8>> {
    input.iter().map(|line| line.bytes().collect()).collect()
}

fn cal_first_part_answer(grid: &[Vec<u8>]) -> usize {
    0
}

fn cal_second_part_answer(grid: &[Vec<u8>]) -> usize {
    0
}
```

เท่านี้เราก็มีโครงโปรแกรมที่พร้อมทำงานกับไฟล์อินพุตจริงแล้ว ต่อไปเราจะใช้ TDD เพื่อพัฒนา `parse_grid`, `find_start`, `cal_first_part_answer` และ `cal_second_part_answer` กัน

---

## Step 1: Parse the Grid

สิ่งแรกที่ต้องทำคือการแปลงอินพุตสตริงให้เป็นกริด (2D vector) ที่เราสามารถทำงานด้วยได้

เริ่มต้นด้วยการเขียนเทสก่อน:

```rust
// src/bin/day7/main.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 15);
        assert_eq!(grid[0][7], b'S');
        assert_eq!(grid[2][7], b'^');
        assert_eq!(grid[4][6], b'^');
        assert_eq!(grid[4][8], b'^');
    }
}
```

รันเทส:

```bash
cd advent-of-code/2025 && cargo test --bin day7
```

```
running 1 test
test tests::test_parse_grid ... ok
```

เทสผ่าน! เพราะเราได้เขียน `parse_grid` ไว้ในโครงหลักแล้ว

---

## Step 2: Find Starting Position

ลำแสงเริ่มต้นที่ตำแหน่ง `S` เราต้องหาว่ามันอยู่ตรงไหนในกริด

เขียนเทส:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_find_start() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
        ];
        let grid = parse_grid(&input);
        let (row, col) = find_start(&grid).unwrap();
        assert_eq!((row, col), (0, 7));
    }

    #[test]
    fn test_find_start_no_s() {
        let input = vec![
            "...............".to_string(),
            "...............".to_string(),
        ];
        let grid = parse_grid(&input);
        assert!(find_start(&grid).is_none());
    }
}
```

รันเทสแล้วพังตามที่คาดไว้ จากนั้นเขียนฟังก์ชัน `find_start`:

```rust
fn find_start(grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == b'S' {
                return Some((row, col));
            }
        }
    }
    None
}
```

รันเทส:

```
running 3 tests
test tests::test_find_start ... ok
test tests::test_find_start_no_s ... ok
test tests::test_parse_grid ... ok
```

ผ่าน!

---

## Step 3: Count Splits - No Splitters

กรณีที่ง่ายที่สุดคือไม่มี splitter เลย ลำแสงจะเคลื่อนที่ลงมาตรง ๆ จนออกจากกริด โดยไม่เกิดการแยกเลย

เขียนเทส:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_no_splitters() {
        let input = vec![
            "S..".to_string(),
            "...".to_string(),
            "...".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 0);
    }

    #[test]
    fn test_no_splitters_single_column() {
        let input = vec![
            "S".to_string(),
            ".".to_string(),
            ".".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 0);
    }
}
```

รันเทสแล้วผ่าน! เพราะฟังก์ชัน `cal_first_part_answer` ที่ return 0 อยู่แล้วตรงกับเคสที่ไม่มี splitter

---

## Step 4: Count Splits - One Splitter

ถ้ามี splitter ตัวเดียว ลำแสงจะแยกครั้งเดียว

เขียนเทส:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_one_splitter() {
        let input = vec![
            "S".to_string(),
            "^".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 1);
    }
}
```

รันเทส:

```
running 6 tests
test tests::test_find_start ... ok
test tests::test_find_start_no_s ... ok
test tests::test_parse_grid ... ok
test tests::test_no_splitters ... ok
test tests::test_no_splitters_single_column ... ok
test tests::test_one_splitter ... FAILED

failures:

---- tests::test_one_splitter stdout ----

thread 'tests::test_one_splitter' (192728) panicked at src/bin/day7/main.rs:95:9:
assertion `left == right` failed
  left: 0
 right: 1
```

เทสพังตามที่คาดไว้ เพราะฟังก์ชันยัง return 0 เสมอ

จากนั้นเขียนฟังก์ชัน `cal_first_part_answer` จริง โดยแทนที่ฟังก์ชันเดิมที่ return 0:

```rust
fn cal_first_part_answer(grid: &[Vec<u8>]) -> usize {
    let start = find_start(grid).expect("grid must have a start position");
    let mut split_count = 0;
    let mut beams = vec![start];
    let mut visited_splitters = vec![vec![false; grid[0].len()]; grid.len()];

    while let Some((row, col)) = beams.pop() {
        let mut r = row + 1;

        while r < grid.len() {
            match grid[r][col] {
                b'^' => {
                    if !visited_splitters[r][col] {
                        visited_splitters[r][col] = true;
                        split_count += 1;
                        if col > 0 {
                            beams.push((r, col - 1));
                        }
                        if col + 1 < grid[0].len() {
                            beams.push((r, col + 1));
                        }
                    }
                    break;
                }
                _ => {
                    r += 1;
                }
            }
        }
    }

    split_count
}
```

รันเทส:

```
running 6 tests
test tests::test_find_start ... ok
test tests::test_one_splitter ... ok
test tests::test_no_splitters ... ok
test tests::test_no_splitters_single_column ... ok
test tests::test_find_start_no_s ... ok
test tests::test_parse_grid ... ok
```

ผ่าน!

เพิ่มเทสอีกเคสสำหรับ splitter ที่ไม่อยู่ตรงแนวเดียวกับ `S`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_one_splitter_offset() {
        let input = vec![
            "S..".to_string(),
            "...".to_string(),
            "..^".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 0);
    }
}
```

รันเทส:

```
running 7 tests
test tests::test_one_splitter_offset ... ok
test tests::test_find_start_no_s ... ok
test tests::test_find_start ... ok
test tests::test_no_splitters ... ok
test tests::test_no_splitters_single_column ... ok
test tests::test_one_splitter ... ok
test tests::test_parse_grid ... ok
```

ผ่าน! เพราะ `^` อยู่ที่ตำแหน่ง (2,2) แต่ลำแสงเคลื่อนที่ลงมาตรง ๆ จาก `S` ที่ (0,0) ในแนวคอลัมน์ 0 จึงไม่เจอ splitter นี้

---

## Step 5: Count Splits - Multiple Splitters in a Line

เมื่อ splitter หลายตัวอยู่ในแนวเดียวกัน ลำแสงจะแยกหลายครั้ง

เขียนเทส:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_two_splitters_in_line() {
        let input = vec![
            "S".to_string(),
            "^".to_string(),
            "^".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 1);
    }

    #[test]
    fn test_three_splitters_in_line() {
        let input = vec![
            "S".to_string(),
            "^".to_string(),
            "^".to_string(),
            "^".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 1);
    }
}
```

รันเทส:

```
running 9 tests
test tests::test_find_start ... ok
test tests::test_find_start_no_s ... ok
test tests::test_no_splitters ... ok
test tests::test_no_splitters_single_column ... ok
test tests::test_one_splitter ... ok
test tests::test_one_splitter_offset ... ok
test tests::test_parse_grid ... ok
test tests::test_three_splitters_in_line ... ok
test tests::test_two_splitters_in_line ... ok
```

ผ่าน! แต่ละเคสได้แค่ 1 split เพราะหลังจาก splitter ตัวแรกที่ (1,0) ลำแสงใหม่ที่แยกไปทางซ้ายและขวาจะออกนอกกริด (เนื่องจากกริดกว้างแค่ 1 คอลัมน์) ทำให้ splitter ตัวถัดไปในแนวเดียวกันไม่ถูกเจอ

---

## Step 6: Count Splits - Diamond Pattern

ลองเคสที่ splitter วางตัวเป็นรูปสามเหลี่ยมหรือเพชร ซึ่งจะทำให้เกิดการแยกที่ซับซ้อนขึ้น

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_diamond_pattern() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
            ".^.^.".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 3);
    }
}
```

รันเทส:

```
running 10 tests
test tests::test_diamond_pattern ... ok
test tests::test_find_start ... ok
test tests::test_find_start_no_s ... ok
test tests::test_no_splitters ... ok
test tests::test_no_splitters_single_column ... ok
test tests::test_one_splitter_offset ... ok
test tests::test_one_splitter ... ok
test tests::test_three_splitters_in_line ... ok
test tests::test_parse_grid ... ok
test tests::test_two_splitters_in_line ... ok
```

ผ่าน!

---

## Step 7: The Full Example

ถึงเวลาทดสอบกับตัวอย่างเต็มที่ให้ไว้ในโจทย์ ซึ่งควรจะได้ผลลัพธ์เป็น 21

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_first_part() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_first_part_answer(&grid), 21);
    }
}
```

รันเทส:

```
running 11 tests
test tests::test_diamond_pattern ... ok
test tests::test_find_start ... ok
test tests::test_find_start_no_s ... ok
test tests::test_first_part ... ok
test tests::test_no_splitters ... ok
test tests::test_no_splitters_single_column ... ok
test tests::test_one_splitter ... ok
test tests::test_one_splitter_offset ... ok
test tests::test_parse_grid ... ok
test tests::test_three_splitters_in_line ... ok
test tests::test_two_splitters_in_line ... ok
```

ผ่าน!

---

## Part 2: Quantum Tachyon Manifold

ปรากฏว่า manifold ที่แท้จริงนั้นเป็นแบบ quantum ซึ่งมีคุณสมบัติพิเศษคือเมื่ออนุภาคชน splitter อนุภาคจะแยกไปทั้งทางซ้ายและทางขวาพร้อมกัน โดยเวลาจะแตกออกเป็นสองเส้นทาง (timelines) ต่างกัน

หน้าที่ของเราคือหาจำนวน timelines ทั้งหมดที่เกิดขึ้นหลังจากอนุภาคหนึ่งตัวเดินทางผ่าน manifold ครบทุกเส้นทางที่เป็นไปได้

ก่อนอื่น implement stub สำหรับ `cal_second_part_answer`:

```rust
fn cal_second_part_answer(grid: &[Vec<u8>]) -> usize {
    0
}
```

จากนั้นเริ่มเขียนเทสทีละขั้น

---

### Step 8: Count Timelines - No Splitters

กรณีที่ง่ายที่สุดคือไม่มี splitter เลย ลำแสงจะเคลื่อนที่ลงมาตรง ๆ จนออกจากกริด โดยมีแค่ 1 timeline

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_second_part_no_splitters() {
        let input = vec![
            "S..".to_string(),
            "...".to_string(),
            "...".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_second_part_answer(&grid), 1);
    }
}
```

รันเทสแล้วพัง! เพราะฟังก์ชันยัง return 0:

```
running 12 tests
test tests::test_second_part_no_splitters ... FAILED
...
```

จากนั้นแก้ไข `cal_second_part_answer` ให้ return 1 สำหรับกรณีที่ไม่มี splitter แต่ยังไม่ต้อง implement จริง:

```rust
fn cal_second_part_answer(grid: &[Vec<u8>]) -> usize {
    1
}
```

รันเทสอีกครั้ง:

```
running 12 tests
test tests::test_second_part_no_splitters ... ok
...
```

ผ่าน!

---

### Step 9: Count Timelines - One Splitter

ถ้ามี splitter ตัวเดียว จะเกิด 2 timelines (แยกซ้าย-ขวา)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_second_part_one_splitter() {
        let input = vec![
            "S".to_string(),
            "^".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_second_part_answer(&grid), 2);
    }
}
```

รันเทส:

```
running 13 tests
test tests::test_second_part_one_splitter ... FAILED
...
```

เทสพังตามที่คาดไว้ จากนั้น implement ฟังก์ชัน `cal_second_part_answer` จริง โดยใช้ recursion และ memoization:

```rust
fn cal_second_part_answer(grid: &[Vec<u8>]) -> usize {
    let start = find_start(grid).expect("grid must have a start position");
    let mut memo = vec![vec![None; grid[0].len()]; grid.len()];
    count_timelines(grid, start.0 + 1, start.1, &mut memo)
}

fn count_timelines(
    grid: &[Vec<u8>],
    r: usize,
    c: usize,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if r >= grid.len() {
        return 1;
    }
    if let Some(val) = memo[r][c] {
        return val;
    }
    let result = match grid[r][c] {
        b'^' => {
            let left = if c > 0 {
                count_timelines(grid, r, c - 1, memo)
            } else {
                1
            };
            let right = if c + 1 < grid[0].len() {
                count_timelines(grid, r, c + 1, memo)
            } else {
                1
            };
            left + right
        }
        _ => count_timelines(grid, r + 1, c, memo),
    };
    memo[r][c] = Some(result);
    result
}
```

รันเทส:

```
running 13 tests
test tests::test_second_part_one_splitter ... ok
test tests::test_second_part_no_splitters ... ok
...
```

ผ่าน!

---

### Step 10: Count Timelines - Multiple Splitters

เพิ่มเทสที่มี splitter หลายตัวในแนวเดียวกัน ควรจะได้ 2 timelines เช่นกันเพราะลำแสงที่แยกไปซ้าย-ขวาจะออกนอกกริด

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_second_part_two_splitters() {
        let input = vec![
            "S".to_string(),
            "^".to_string(),
            "^".to_string(),
            ".".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_second_part_answer(&grid), 2);
    }
}
```

รันเทส:

```
running 14 tests
test tests::test_second_part_no_splitters ... ok
test tests::test_second_part_one_splitter ... ok
test tests::test_second_part_two_splitters ... ok
...
```

ผ่าน!

---

### Step 11: Count Timelines - Diamond Pattern

เคสที่ splitter วางตัวเป็นรูปเพชร ควรจะได้ 4 timelines

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_second_part_diamond_pattern() {
        let input = vec![
            "..S..".to_string(),
            ".....".to_string(),
            "..^..".to_string(),
            ".....".to_string(),
            ".^.^.".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_second_part_answer(&grid), 4);
    }
}
```

รันเทส:

```
running 15 tests
test tests::test_second_part_diamond_pattern ... ok
test tests::test_second_part_no_splitters ... ok
test tests::test_second_part_one_splitter ... ok
test tests::test_second_part_two_splitters ... ok
...
```

ผ่าน!

---

### Step 12: Count Timelines - Full Example

ถึงเวลาทดสอบกับตัวอย่างเต็ม ซึ่งควรจะได้ 40 timelines

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...

    #[test]
    fn test_second_part() {
        let input = vec![
            ".......S.......".to_string(),
            "...............".to_string(),
            ".......^.......".to_string(),
            "...............".to_string(),
            "......^.^......".to_string(),
            "...............".to_string(),
            ".....^.^.^.....".to_string(),
            "...............".to_string(),
            "....^.^...^....".to_string(),
            "...............".to_string(),
            "...^.^...^.^...".to_string(),
            "...............".to_string(),
            "..^...^.....^..".to_string(),
            "...............".to_string(),
            ".^.^.^.^.^...^.".to_string(),
            "...............".to_string(),
        ];
        let grid = parse_grid(&input);
        assert_eq!(cal_second_part_answer(&grid), 40);
    }
}
```

รันเทส:

```
running 16 tests
test tests::test_second_part ... ok
test tests::test_second_part_diamond_pattern ... ok
test tests::test_second_part_no_splitters ... ok
test tests::test_second_part_one_splitter ... ok
test tests::test_second_part_two_splitters ... ok
...
```

ผ่าน!

## รันกับอินพุตจริง

เมื่อเทสผ่านทั้งหมดแล้ว เราสามารถรันโปรแกรมกับไฟล์อินพุตจริงได้เลย:

```bash
cd advent-of-code/2025 && cargo run --bin day7 -- src/bin/day7/input.txt
```

โปรแกรมจะอ่านแผนผังจากไฟล์ คำนวณจำนวนการแยกลำแสงและจำนวน timelines และพิมพ์ผลลัพธ์ออกมา

```
First part answer: 1592
Second part answer: 17921968177009
```
