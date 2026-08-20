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
}

fn parse_grid(input: &[String]) -> Vec<Vec<u8>> {
    input.iter().map(|line| line.bytes().collect()).collect()
}

fn cal_first_part_answer(grid: &[Vec<u8>]) -> usize {
    0
}
```

เท่านี้เราก็มีโครงโปรแกรมที่พร้อมทำงานกับไฟล์อินพุตจริงแล้ว ต่อไปเราจะใช้ TDD เพื่อพัฒนา `parse_grid`, `find_start` และ `cal_first_part_answer` กัน

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
    fn test_full_example() {
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
test tests::test_full_example ... ok
test tests::test_no_splitters ... ok
test tests::test_no_splitters_single_column ... ok
test tests::test_one_splitter ... ok
test tests::test_one_splitter_offset ... ok
test tests::test_parse_grid ... ok
test tests::test_three_splitters_in_line ... ok
test tests::test_two_splitters_in_line ... ok
```

ผ่าน! ครบ 21 ครั้งตามที่โจทย์กำหนด

---

## รันกับอินพุตจริง

เมื่อเทสผ่านทั้งหมดแล้ว เราสามารถรันโปรแกรมกับไฟล์อินพุตจริงได้เลย:

```bash
cd advent-of-code/2025 && cargo run --bin day7 -- path/to/input.txt
```

โปรแกรมจะอ่านแผนผังจากไฟล์ คำนวณจำนวนการแยกลำแสง และพิมพ์ผลลัพธ์ออกมา

---

## สรุป

เราใช้ TDD เพื่อแก้โจทย์ Advent of Code 2025 Day 7 ด้วยภาษา Rust โดยเริ่มจากขั้นตอนง่าย ๆ ไปจนถึงตัวอย่างเต็ม:

- **Step 1:** สร้างฟังก์ชัน `parse_grid` สำหรับแปลงอินพุตเป็นกริด
- **Step 2:** สร้างฟังก์ชัน `find_start` สำหรับหาตำแหน่งเริ่มต้นของลำแสง
- **Step 3:** เริ่มต้น `cal_first_part_answer` สำหรับกรณีที่ไม่มี splitter
- **Step 4-6:** เพิ่มความซับซ้อนทีละน้อยด้วย splitter หนึ่งตัว หลายตัว และรูปแบบต่าง ๆ
- **Step 7:** ทดสอบกับตัวอย่างเต็มที่ได้ 21 ครั้ง

แนวคิดสำคัญที่ได้จาก kata นี้คือการจำลองลำแสง (beam simulation) ด้วยการเก็บตำแหน่งของลำแสงทั้งหมดใน `Vec` และประมวลผลทีละตัว เมื่อเจอ splitter ก็เพิ่มลำแสงใหม่เข้าไปในลิสต์ ทำแบบนี้ไปเรื่อย ๆ จนกระทั่งลำแสงทั้งหมดออกจากกริด

---

ตัวอย่างโค้ดฉบับเต็มสามารถดูได้ที่ [GitHub](https://github.com/nomkhonwaan/advent-of-code-2025/tree/main/src/bin/day7)