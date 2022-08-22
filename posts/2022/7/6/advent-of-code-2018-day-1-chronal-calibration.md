---
title: Advent of Code 2018 Day 1 - Chronal Calibration
publish_date: 2022-07-06
---

เห็นบล็อก Advent of Code 2018 ณ ตอนปี 2022 อาจจะเผลอคิดในใจกันว่าช้าไปหรือเปล่า (ฮา) บังเอิญได้มีโอกาสอ่านบล็อก [Learning Rust in 2020](https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md#tldr) และมีการพูดถึง Advent of Code 2018 ว่าเป็นจุดเริ่มต้นที่ดีสำหรับใช้เรียนรู้ภาษา Rust

ส่วนตัวหัดเขียน Rust มาสักระยะแล้วแต่ยังรู้สึกไม่ชินมือเท่าไร ได้โอกาสลองเล่น Advent of Code 2018 พร้อมกับเรียนรู้ภาษา Rust ไปด้วยดีกว่า

## สารบัญ

- [Advent of Code 2018 Day 1 - Chronal Calibration](/2022/7/6/advent-of-code-2018-day-1-chronal-calibration)
- [Advent of Code 2018 Day 2 - Inventory Management System](/2022/7/28/advent-of-code-2018-day-2-inventory-management-system)
- [Advent of Code 2018 Day 3 - No Matter How You Slice it](/2022/8/5/advent-of-code-2018-day-3-no-matter-how-you-slice-it)
- [Advent of Code 2018 Day 4 - Repose Record](/2022/8/22/advent-of-code-2018-day-4-repose-record)

## TL;DR

[GitHub](https://github.com/nomkhonwaan/nomkhonwaan/blob/main/advent-of-code/2018/day_1_chronal_calibration.rs)

---

เรื่องราวเริ่มต้นเมื่อมีคนพยายามจะแก้ไขประวัติศาสตร์ของซานต้าเมื่อ 500 ปีก่อน เอลฟ์สาวประจำสถานีวิจัยและตรวจสอบความผิดปกติเป็นคนเล่า เธอต้องการให้เราย้อนเวลากลับไปแก้ไขเรื่องราวเหล่านี้

"ข่าวดีประวัติศาสร์ยังไม่ได้บิดเบือนกับเวลาในยุคปัจจุบันและพวกเรามีอุปกรณ์สำหรับแก้ไขปัญหานี้แล้ว!" — _เอลฟ์สาว_

หล่อนจัดการสวมเจ้าอุปกรณ์ที่ว่านี้บนข้อมือเรา

"อุปกรณ์นี้จะช่วยเราแก้ไขความผิดปกติ มันถูกตั้งค่าให้ส่งตัวคุณไปเมื่อ 500 ปีก่อน นี้คือวิธีที่ดีที่สุดเท่าที่เราจะทำได้ ณ ตอนนี้" — _เอลฟ์สาว_

"ข่าวร้ายคือเราพบว่ามีความผิดปกติถึง 50 อย่าง แต่ไม่ต้องกังวลไปเจ้าอุปกรณ์นี้จะซ่อมแซมความผิดปกติได้ด้วยการใช้ดวงดาว เอาละได้เวลาแล้วขอให้คุณโช.." — _เอลฟ์สาว_

หล่อนพูดยังไม่ทันจบก็จัดการกดปุ่มที่อุปกรณ์บนข้อมือเราทันที เจ้าอุปกรณ์ที่เอลฟ์สาวให้เรามาสามารถใช้แก้ไขความผิดปกตินี้ได้ด้วยการใช้ดวงดาว ซึ่งในทุกครั้งที่เราแก้ไขปัญหาเราจะได้รับดวงดาวมาหนึ่งดวง และในทุก ๆ วันจะมีปัญหาสองข้อให้ต้องแก้ไขด้วยกัน

คุณถูกส่งต้วมาในอดีตเมื่อ 500 ปีก่อนเรียบร้อยแล้วและเมื่อก้มมองดูที่อุปกรณ์ก็พบว่ามีข้อความปรากฎอยู่ดังนี้


_"Error: Device must be calibrated before first use. Frequency drift detected. Cannot maintain destination lock."_

---

เอาละมาถึงส่วนที่ต้องทำโจทย์กันแล้ว

โจทย์เล่าว่าทุกครั้งที่คลื่นความถี่เปลี่ยน ตัวอุปกรณ์จะแสดงตัวเลขความถี่เรียงลำดับกันออกมาแบบนี้ `+1, -2, +3, +1` ความต้องการของโจทย์คือให้โปรแกรมรับค่าลำดับตัวเลขความถี่ทั้งหมดและเอาจำนวนมารวมกัน

หน้าเว็บจะมีที่ให้ดาวน์โหลดอินพุตของโปรแกรมชื่อว่า "input.txt" ได้เวลาลงมือเขียนโปรแกรมกัน

---

เริ่มจากสร้างโปรเจกต์ด้วยคำสั่ง `cargo new advent-of-code-2018` จะได้โปรเจกต์พร้อมโครงสร้างภายในแบบนี้

```
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

จัดการเปิดไฟล์ main.rs แล้วเพิ่มส่วนของการรับอาร์กิวเมนต์แบบนี้

```rust
use std:: {env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: &String = &args[1];
}
```

จากนั้นมาเขียนฟังก์ชันสำหรับเปิดไฟล์เพื่ออ่านทีละบรรทัดตามตัวอย่าง [`read_lines`](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html) แบบนี้

```rust
use std::{env, fs, io, io::BufRead, path};

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
```

เมื่อได้ฟังก์ชัน `read_lines` แล้วมาเติมส่วนของการแปลงค่าสตริงเป็นตัวเลขที่ `main` แบบนี้

```rust
use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut first_part_answer = 0isize;

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                // convert string to signed integer
                let current_frequency: isize = line.parse().expect("invalid number");
                first_part_answer += current_frequency;
            }
        }
    }

    println!("first part answer is: {}", first_part_answer);
}
```

เท่านี้ก็ได้คำตอบของพาร์ทแรกแล้ว

---

มาต่อกันที่พาร์ทสอง โจทย์เล่าว่าอุปกรณ์เนี่ยมันแสดงความถี่เป็นซ้ำ ๆ กันเป็นรูปแบบ โจทย์ต้องการให้หาว่าผลรวมของความถี่ใดที่ซ้ำกันสองก่อนผลรวมอื่น ยกตัวอย่างเช่น

ลำดับ `+1, -1` พบว่าผลรวม `0` ซ้ำกันสองครั้งก่อนผลรวมอื่น ๆ

```
1. +1, -1
2. +1, -1, +1, -1
```

ลำดับ `+3, +3, +4, -2, -4` พบว่าผลรวม `10` ซ้ำกันสองครั้งก่อนผลรวมอื่น ๆ
```
1. +3, +3, +4 
2. +3, +3, +4, -2, -4, +3, +3
```

ลำดับ `-6, +3, +8, +5, -6` พบว่าผลรวม `5` ซ้ำกันสองครั้งก่อนผลรวมอื่น ๆ

```
1. -6, +3, +8
2. -6, +3, +8, +5, -6, -6, +3, +8, +5, -6, -6, +3
```

ลำดับ `+7, +7, -2, -7, -4` พบว่าผลรวม `14` ซ้ำกันสองครั้งก่อนผลรวมอื่น ๆ

```
1. +7, +7
2. +7, +7, -2, -7, -4, +7, +7, -2, -7, -4, +7, +7, -2
```

มาลงมือเขียนโปรแกรมเพื่อโจทย์กัน โปรแกรมในพาร์ทมีการรับไฟล์อินพุตเข้ามาและทำการแปลงตัวเลขจาก `String` เป็น `isize` จากนั้นจึงรวมตัวเลขทั้งหมดเข้าด้วยกันเพื่อหาผลลัพธ์สุดท้าย

แต่ในพาร์ทสองนี้จะต่างออกไปสักเล็กน้อยเนื่องจากต้องมีการเก็บผลรวมของตัวเลขก่อนหน้าเพื่อเอามาหาว่าผลรวมนั้นเคยได้มาก่อนหรือไม่ นอกจากนี้แล้วโปรแกรมต้องสามารถวนกลับไปอ่านตั้งแต่บรรทัดแรกสุดใหม่ได้ด้วยจนกว่าจะพบตัวเลขผลรวมที่ซ้ำกันสองครั้ง

สร้างฟังก์ชันสำหรับวนลูปเพื่อหาผลรวมและตรวจสอบว่าผลรวมนั้นเคยมีหรือยังแบบนี้

```rust
fn find_first_reaches_twice(frequency: &Vec<isize>) -> isize {
    let mut resulting_frequency = 0isize;
    let mut reaches: Vec<isize> = vec![0]; // first reaches is zero

    loop {
        for current_frequency in frequency {
            resulting_frequency += current_frequency;
            if reaches.contains(&resulting_frequency) {
                return resulting_frequency;
            }
            reaches.push(resulting_frequency);
        }
    }
}
```

ต่อมาแก้ไข `main` ให้รวบรวมตัวเลขความถี่ใส่ไว้ที่ตัวแปร `frequency` เพื่อส่งต่อให้กับ `find_first_reaches_twice` ฟังก์ชัน

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut frequency: Vec<isize> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                // convert string to signed integer
                let current_frequency: isize = line.parse().expect("invalid number");
                frequency.push(current_frequency);
            }
        }
    }

    let second_part_answer = find_first_reaches_twice(&frequency);
    println!("second part answer is: {}", second_part_answer);
}
```

ใช้เวลารันสักพักใหญ่ ๆ ก็จะได้คำตอบของพาร์ทสอง

---
#advent-of-code #rust