---
title: Advent of Code 2018 Day 1 - Chronal Calibration
publish_date: 2022-7-15
---

เห็นบล็อก Advent of Code 2018 ณ ตอนปี 2022 อาจจะเผลอคิดในใจกันว่าช้าไปหรือเปล่า (ฮา) ต้องออกตัวก่อนว่าเล่น Advent of Code อยู่แล้วเป็นประจำทุกปีเพียงแค่ไม่ได้เอามาเขียนเล่าเป็นบล็อก 

แต่บังเอิญได้มีโอกาสอ่านบล็อก [Learning Rust in 2020](https://github.com/pretzelhammer/rust-blog/blob/master/posts/learning-rust-in-2020.md#tldr) และมีการพูดถึง Advent of Code 2018 ว่าเป็นจุดเริ่มต้นที่ดีสำหรับใช้เรียนรู้ภาษา Rust

ส่วนตัวหัดเขียน Rust มาสักระยะแล้วแต่ยังรู้สึกไม่ชินมือเท่าไร ถือโอกาสลองกลับมาเล่น Advent of Code 2018 ด้วยภาษา Rust เลยดีกว่า

## สารบัญ

- Advent of Code 2018 Day 1 - Chronal Calibration

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

    println!("processing input file: {}", input);
}
```

ลองสั่งรันด้วย `cargo run -- input.txt` จะได้ผลลัพธ์แบบนี้

```
process input file: input.txt
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

    println!("process input file: {}", input);

    let mut sum: isize = 0;

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let freq: isize = line.parse().expect("invalid number");
                // accumulate the frequency to sum variable
                sum += freq;
            }
        }
    }

    println!("the answer is: {}", sum);
}
```

เท่านี้ก็ได้คำตอบของพาร์ทแรกในวันที่หนึ่งแล้ว

---

ถ้าอ่านแล้วยังไม่ค่อยเข้าใจสามารถเข้าไปดูหรือดาวน์โหลดตัวอย่าง [โค้ดพาร์ทแรก](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c8d1896914f6c4d82a5f84d133af3395) และ [โค้ดพาร์ทสอง]() มาลองเล่นจะช่วยให้เข้าใจได้ง่ายขึ้น