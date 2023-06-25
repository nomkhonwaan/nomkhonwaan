---
title: Advent of Code 2018 Day 5 - Alchemical Reduction
publish_date: 2023-06-24
tags: ['advent-of-code', 'rust']
---

คุณสามารถแอบเข้าไปในห้องแล็บได้สำเร็จ การพัฒนาชุดสูทต้นแบบของเอลฟ์มีความก้าวหน้าพอสมควร แต่ยังคงติดปัญหาเรื่องการลดขนาดของชุดสูทที่จะใช้งานจริง

## สารบัญ

- [Advent of Code 2018 Day 1 - Chronal Calibration](/2022/7/6/advent-of-code-2018-day-1-chronal-calibration)
- [Advent of Code 2018 Day 2 - Inventory Management System](/2022/7/28/advent-of-code-2018-day-2-inventory-management-system)
- [Advent of Code 2018 Day 3 - No Matter How You Slice it](/2022/8/5/advent-of-code-2018-day-3-no-matter-how-you-slice-it)
- [Advent of Code 2018 Day 4 - Repose Record](/2022/8/22/advent-of-code-2018-day-4-repose-record)
- [Advent of Code 2018 Day 5 - Alchemical Reduction](/2023/6/24/advent-of-code-2018-day-5-alchemical-reduction)

## TL;DR

[GitHub](https://github.com/nomkhonwaan/nomkhonwaan/blob/main/advent-of-code/2018/day_5_alchemical_reduction.rs)

---


ถึงแม้ว่าเทคโนโลยีในปี ค.ศ. 1518 จะสามารถแก้ปัญหานี้ได้ในที่สุด แต่ว่าตอนนี้คุณสามารถช่วยพวกเขาได้ คุณตรวจสอบองค์ประกอบทางเคมีของวัสดุที่ใช้ทำชุดสูทและพบว่ามันถูกสร้างขึ้นจากสายโพลิเมอร์ที่ยาวมาก

---

มาถึงส่วนของโจทย์วันนี้

พอลิเมอร์เกิดจากยูนิตเล็ก ๆ ซึ่งจะทำปฏิกิริยาเมื่อยูนิตสองตัวที่เป็นประเภทเดียวกันถูกกระตุ้นและมีขั้วตรงข้ามถูกทำลาย ประเภทของยูนิตแทนที่ด้วยตัวอักษรเช่น `r` และขั้วของยูนิตแทนที่ด้วยตัวอักษรพิมพ์ใหญ่เช่น `R` เรียกยูนิตแบบนี้ว่าเป็นยูนิตประเภทเดียวกันที่มีขั้วตรงข้าม หรืออีกตัวอย่างคือ `r` กับ `s` ที่เป็นยูนิตคนละประเภทกันและจะไม่ทำปฏิกิริยาต่อกัน

ตัวอย่างเช่น

- `aA`, `a` และ `A` จะทำปฏิกิริยากันและสลายไป
- `abBA`, `bB` จะทำปฏิกิริยากันและสลายไปเหลือ `aA` ซึ่งจะเข้าเงื่อนไขด้านบนคือ `a` และ `A` จะทำปฏิกิริยากันและสลายไป
- `abAB`, ไม่เกิดอะไรขึ้นเนื่องจากไม่มียูนิตประเภทเดียวกันติดกัน
- `aabAAB`, ไม่เกิดอะไรขึ้นเนื่องจาก `aa` และ `AA` ที่อยู่ติดกันเป็นขั้วเดียวกัน

โจทย์ต้องการให้หาความยาวของสายพอลิเมอร์หลังจากเกิดปฏิกิริยาแล้ว ดูแล้วไม่น่ายากเริ่มต้นด้วยการดาวน์โหลดอินพุตไฟล์เข้ามาในโปรแกรมก่อน เนื่องจากรอบนี้อินพุตจะเป็นข้อความยาว ๆ เพียงบรรทัดเดียวดังนั้นตรวจสอบให้ดีว่าอินพุตต้อง*ไม่มี*บรรทัดเกินมา

รอบนนี้จะเปลี่ยนฟังก์ชันที่ใช้สำหรับอ่านไฟล์สักเล็กน้อยจากเดิมที่อ่านทีละบรรทัดเปลี่ยนเป็นอ่านทีเดียวทั้งไฟล์​แทน แบบนี้

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let polymer = read_file(input).unwrap();

    ...
}

fn read_file<P: AsRef<path::Path>>(path: P) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
```

จากนั้นเขียนฟังก์ชัน `reduce` สำหรับวนลูปสายพอลิเมอร์เพื่อทำการตัดยูนิตที่เกิดปฏิกิริยาออก โดยเงื่อนไขของฟังก์ชันคือจะวนลูปเรื่อย ๆ จนกว่าจะไม่เกิดปฏิกิริยาแล้วถึงจะหยุดและส่งยูนิตที่เหลืองทั้งหมดกลับ

```rust
fn reduce(&polymer: String) -> String {
    let mut units: Vec<char> = polymer.chars().collect();

    loop {
        let mut reacted = false;

        for i in 0..units.len() - 1 {
            if react(&units[i], &units[i + 1]) {
                // Remove the current and next item from the units vector.
                units.remove(i);
                // The above remove function makes the next index changed,
                // so the next index will be i instead of i + 1.
                units.remove(i);

                reacted = true;
                break;
            }
        }

        // Loop until no reaction, then break.
        if !reacted {
            break;
        }
    }

    units.iter().collect()
}
```

และสร้างสำหรับฟังก์ชันสำหรับใช้ตรวจสอบการเกิดปฏิกิริยาระหว่างยูนิตที่อยู่ติดกัน โดยมีเงื่อนไขคือถ้ายูนิตคู่นั้นมีขั้วแบบเดียวกันหรือไม่ใช่ประเภทเดียวกันจะไม่เกิดปฏิกิริยาใด ๆ 

```rust
fn react(n: &char, m: &char) -> bool {
    // Nothing happens when they are the same type and their polarities match.
    // Example: aa, AA, CC
    if n == m {
        return false 
    }
    // At this statement, they might be the same type or not,
    // But their polarities are not the same for sure.
    return n.to_ascii_lowercase() == m.to_ascii_lowercase();
}
```

จากนั้นเรียกใช้งานฟังก์ชัน `reduce` จาก `main` ก็จะได้คำตอบของพาร์ทที่หนึ่ง

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let polymer = read_file(input).unwrap();

    println!("first part answer is: {}", reduce(&polymer).len());
}
```

---

ในพาร์ทที่สองโจทย์ต้องการให้หาว่าถ้าหากลบยูนิตใด ๆ ก็ตามออกแล้วสามารถทำให้สายพอลิเมอร์มีขนาดที่สั้นที่สุดได้โดยไม่สนใจว่าขั้วจะตรงข้ามกันหรือไม่ 

ตัวอย่างเช่น `dabAcCaCBAcCcaDA`

- ถ้าลบ `A/a` ออกจะได้ `dbcCCBcCcD` หลังจากที่ปล่อยให้เกิดปฏิกิริยาแล้วจะเหลือ `dbCBcD` ซึ่งมีความยาว 6
- ถ้าลบ `B/b` ออกจะได้ `daAcCaCAcCcaDA` หลังจากที่ปล่อยให้เกิดปฏิกิริยาแล้วจะเหลือ `daCAcaDA` ซึ่งมีความยาว 8
- ถ้าลบ `C/c` ออกจะได้ `dabAaBAaDA` หลังจากที่ปล่อยให้เกิดปฏิกิริยาแล้วจะเหลือ `daDA` ซึ่งมีความยาว 4
- ถ้าลบ `D/d` ออกจะได้ `abAcCaCBAcCcaA` หลังจากที่ปล่อยให้เกิดปฏิกิริยาแล้วจะเหลือ `abCBAc` ซึ่งมีความยาว 6

จากตัวอย่างข้างต้นหมายความว่าถ้าลบ `C/c` ออกจากจะทำให้ได้สายพอลิเมอร์ที่มีขนาดสั้นที่สุด

ด้วยเงื่อนไขนี้ทำให้จำเป็นต้องมีการเพิ่มโค้ดส่วนของการตัดยูนิตออกก่อนที่จะส่งเข้าฟังก์ชัน `reduce` โดยเพิ่มโค้ดเข้าไปที่ `main` แบบนี้

```rust

fn main() {
    ...

    let instances = "abcdefghijklmnopqrstuvwxyz";
    let mut shortest_polymer_len = polymer.len();

    for instance in instances.chars().collect::<Vec<char>>() {
        let mut produced_polymer = polymer.clone();
        produced_polymer = produced_polymer.replace(instance, "");
        produced_polymer = produced_polymer.replace(instance.to_ascii_uppercase(), "");

        let polymer_len = reduce(&produced_polymer).len();
        if polymer_len < shortest_polymer_len {
            shortest_polymer_len = polymer_len;
        }
    }

    println!("second part answer is: {}", shortest_polymer_len);
}
```

จากโค้ดคือจะทำการวนลูปทั้งหมด 26 ครั้งตามจำนวนตัวอักษร เพื่อทดลองดูว่าหลังจากลบยูนิตดังกล่าวออกจากสายพอลิเมอร์แล้วจะได้ขนาดเท่าไรหลังจากเกิดปฏิกิริยา เท่านี้ก็จะได้คำตอบของพาร์ทที่สองแล้ว

---

### อัพเดท

เนื่องจากโค้ดเดิมโปรแกรมจะทำงานตามลำดับ (sequential) ในการทดลองตัดยูนิตออก ก่อนที่จะเอาไปเช็คการเกิดปฏิกิริยา ซึ่งจำนวนยูนิตที่ต้องทดลองตัดมีทั้งหมด 26 ตัวอักษรทำให้โปรแกรมต้องรอการทำงานแต่ละรอบให้จบก่อนถึงจะสามารถทำงานรอบถัดไปได้ ซึ่งในความเป็นจริงนั้นแต่ละตัวอักษรที่จะเอามาทดลองตัดเป็นอิสระต่อกัน (independent) เลยออกมาเป็นโค้ดเวอร์ชันปรับปรุงแบบนี้ด้วยการใช้งาน `tokio` และ `async`

```rust
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let polymer = read_file(input).unwrap();

    println!("first part answer is: {}", reduce(&polymer).len());

    let instances = "abcdefghijklmnopqrstuvwxyz";
    let mut tasks = vec![];

    for instance in instances.chars().collect::<Vec<char>>() {
        let task = tokio::task::spawn(produce(polymer.clone(), instance));
        tasks.push(task);
    }

    let shortest_polymer_len = join_all(tasks)
        // At this moment, you will get Vec<Result<usize, JoinError>>
        .await 
        .into_iter()
        // Filter only success result (it should success for all)
        .filter_map(Result::ok)
        // Transform the Vec<Result<usize, JoinError>> to Vec<usize>
        .collect::<Vec<usize>>()
        .into_iter()
        // Find the minimum among the result
        .min();

    println!(
        "second part answer is: {}",
        // Get the minimum result, otherwise use the origin length as the result
        shortest_polymer_len.unwrap_or(polymer.len())
    );
}
```

เริ่มจากเพิ่ม annotation `#[tokio::main]` และ `async` เข้าไปที่ฟังก์ชัน `main` เพื่อเรียกใช้ macro ของ `tokio` ที่จะไปแปลงฟังก์ชัน `main` อีกทีหนึ่ง ซึ่งจริง ๆ ถ้าไม่เรียกใช้ macro ก็สามารถเรียก `tokio::runtime::Runtime` เองได้เช่นกัน

จากนั้นในลูป `for` แทนที่จะทำงานตรง ๆ เปลี่ยนมาเรียกฟังก์ชัน `produce` โดยการสั่ง `tokio::task::spawn` ออกไปอีก thread แทน

ซึ่งเจ้าฟังก์ชัน `produce` ก็จะทำงานคล้ายกับที่เขียนไว้ในฟังก์ชัน `main` เดิมเพียงแต่เปลี่ยนมาส่งค่าความยาวของสายพอลิเมอร์กลับ แทนที่จะ assign ใส่ตัวแปร `shortest_polymer_len` ตรง ๆ

```rust
async fn produce(mut polymer: String, instance: char) -> usize {
    polymer = polymer.replace(instance, "");
    polymer = polymer.replace(instance.to_ascii_uppercase(), "");

    reduce(&polymer).len()
}
```

และนี้คือผลลัพธ์ที่ได้หลังจากเปลี่ยนมาทำงานแบบ async จากเดิมที่ใช้เวลารันโดยเฉลี่ยอยู่ที่ประมาณ​ 80s

```text
first part answer is: 9704
second part answer is: 6942

________________________________________________________
Executed in   81.27 secs    fish           external
   usr time   80.53 secs    0.13 millis   80.53 secs
   sys time    0.15 secs    1.17 millis    0.15 secs
```

เหลือเวลารันโดยเฉลี่ยอยู่ที่ 15s ลดลงไปกว่า 5 เท่า!

```text
first part answer is: 9704
second part answer is: 6942

________________________________________________________
Executed in   15.22 secs    fish           external
   usr time   98.24 secs    0.10 millis   98.24 secs
   sys time    0.13 secs    1.18 millis    0.13 secs
```