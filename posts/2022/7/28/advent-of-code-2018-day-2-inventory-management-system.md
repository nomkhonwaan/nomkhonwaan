---
title: Advent of Code 2018 Day 2 - Inventory Management System
publish_date: 2022-07-28
---

ในวันที่สองตัวเราหยุดการเดินทางผ่านกาลเวลาแล้ว หลังจากพักหายใจสักครู่ก็ก้มมองดูเจ้าอุปกรณ์ที่อยู่บนข้อมือที่ได้รับมา ที่หน้าจอแสดงข้อความว่าถึงที่หมายปลายทางแล้วพร้อมกับแจ้งปี ค.ศ. 1518 และสถานที่ปัจจุบันคือ​ _ตู้เอนกประสงค์แห่งสำนักงานขั้วโลกเหนือหมายเลข 83N10_

## สารบัญ

- [Advent of Code 2018 Day 1 - Chronal Calibration](/2022/7/6/advent-of-code-2018-day-1-chronal-calibration)
- Advent of Code 2018 Day 2 - Inventory Management System


## TL;DR

[GitHub](https://github.com/nomkhonwaan/nomkhonwaan/blob/main/advent-of-code/2018/day_2_inventory_management_system.rs)

---

คุณได้ยินเสียงฝีเท้าและเสียงพูดคุยอยู่ด้านนอก

"...ฉันเองก็ไม่มั่นใจเหมือนกัน แต่คนส่วนมากมีปล่องไฟอยู่ที่บ้าน ไม่แน่เขาอาจจะอาศัยช่องนั่นมุดตัวผ่านเข้าไปได้?" — _เสียงลึกลับ_

"อันที่จริงแล้วเรากำลังสร้างชุดแบบใหม่ที่ช่วยให้เขาสามารถผ่านที่แคบแบบนั้นได้ แต่เมื่อไม่กี่วันก่อนฉันได้ยินว่าตัวอย่างผ้า, แบบแปลน, และทุกสิ่ง! อยู่ดี ๆ ทุกคนในทีมก็ลืมมันไปหมดเหมือนไม่เคยเกิดขึ้นมาก่อน!" — _เสียงลึกลับอีกเสียง_

"Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.

"อันที่จริงแล้วพวกเขาน่าจะยังมีผ้าเหลืออยู่ที่โกดังไม่ใช่หรือ? มันน่าจะถูกเก็บรวมกันอยู่ในกล่องที่มีรหัสใกล้เคียงกันแต่น่าเสียดายมันคงจะเป็นเรื่องยากที่ต้องค้นหากล่องสองใบที่มีรหัสใกล้เคียงกันจากกล่องจำนวนมหาศาลขนาดนั้น..." เสียงลึกลับทั้งสองเสียงเดินห่างออกไปไกลเกินกว่าที่จะได้ยินประโยคที่เหลือทั้งหมด

ในคืนนั้นเอง, คุณแอบย่องเข้าไปที่โกดังเพื่อที่จะค้นหากล่องสอบใบที่ว่านั่นอย่างระมัดระวัง ใครจะรู้ถ้าเกิดคุณถูกเจอตัวเข้ามันอาจจะเปลี่ยนแปลงเหตุการณ์อะไรในอนาคตเข้าก็ได้ คุณใช้อุปกรณ์ที่ข้อมือสแกนหากล่องที่_น่าจะใช่_ทั้งหมดได้ออกมาจำนวนหนึ่ง

---

มาถึงส่วนของโจทย์ในวันนี้แล้ว

โจทย์ต้องการให้หาว่าในหมายเลขกล่องที่ได้มามีตัวอักษรใดบ้างที่ซ้ำกันสองหรือสามครั้งเช่น

- `abcdef` ไม่มีตัวอักษรใดที่ซ้ำสองหรือสามครั้งเลย
- `bababc` มี `a` สองครั้งและ `b` สามครั้ง
- `abbcde` มี `a` สองครั้งแต่ไม่มีตัวอักษรใดที่ซ้ำสามครั้ง
- `abcccd` มี `c` สามครั้งแต่ไม่มีตัวอักษรใดที่ซ้ำสองครั้ง
- `aabcdd` มี `a` และ `d` ที่ซ้ำสองครั้ง
- `abcdee` มี `e` สองครั้ง
- `ababab` มี `a` และ `b` ที่ซ้ำสามครั้ง

จากรายการข้างต้นมีสี่รหัสที่มีตัวอักษรซ้ำสองครั้งคือ `bababc`,​ `abbcde`, `aabcdd` และ `abcdee` กับรหัสที่มีตัวอักษกซ้ำสามครั้งคือ `bababc` (นับอีกรอบเพราะมีทั้งสองและสาม), `abcccd` และ `ababab` จากนั้นเอาจำนวนของรหัสที่ซ้ำสองและสามครั้งมาคูณกัน จากตัวอย่างจะได้เป็น `4 * 3 = 12` 

---

เหมือนกันกับวันแรกเริ่มจากดาวน์โหลดอินพุต "input.txt" จากนั้นใช้ฟังก์ชันเดิมเพื่ออ่านไฟล์ทีละบรรทัด

```rust
use std::{env, fs, io, io::BufRead, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut list_of_box_ids: Vec<BoxId> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                // TODO: Parse each line into `BoxId` here...
            }
        }
    }
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
```

ถัดมาสร้าง `BoxId` เพื่อใช้สำหรับเก็บข้อมูลหมายเลขกล่อง ถ้าสังเกตจะเห็นว่าหมายเลขกล่องจะถูกเรียงลำดับอันนี้เพื่อใช้เวลาหาตัวอักษรที่ซ้ำกันสองและสามครั้งจะทำให้ง่ายกว่าการนับด้วยการวนลูปซ้อนกันสองลูป

```rust
/// Represent each line of the ID
#[derive(Debug, Default)]
struct BoxId {
    sorted_chars: Vec<char>,
}

impl BoxId {
    fn from_string(s: String) -> Self {
        let mut c: Vec<char> = s.chars().collect();
        // sort the given id ascending
        c.sort();
        BoxId { sorted_chars: c }
    }
}
```

จากนั้นมาสร้างฟังก์ชันสำหรับนับตัวอักษรที่ซ้ำกันสองครั้งแบบนี้ จากโค้ดเริ่มด้วยการวนลูปตัวอักษรที่ละตัวด้วยคำสั่ง `iter` และให้ส่งค่าอินเด็กซ์กับตัวอักษรกลับมาด้วย `enumerate` 

จากนั้นเช็คว่าถ้า `i + ` มากกว่า `max` ของจำนวนตัวอักษรให้ออกจากลูปเพราะถือว่าวนจนจบทั้งหมดแล้ว ถัดมาจะเป็นการตรวจสอบว่าตัวอักษรปัจจุบันเป็นตัวเดียวกับตัวอักษรในลำดับถัดไปหรือไม่ ถ้าไม่ใช่ให้วนลูปต่อทันทีไม่ต้องตรวจสอบเพิ่มเติมแล้ว

ในกรณีที่ตัวอักษรลำดับถัดไปเป็นค่าเดียวกันต้องมีการตรวจสอบเพิ่มเติมอีกคือต้องไม่ใช่การซ้ำกันสามครั้ง​ โดยตรวจสอบกับตัวอักษรในลำดับถัดไปอีกสองตำแหน่ง และตัวอักษรในลำดับก่อนหน้าหนึ่งตำแหน่ง

```rust
fn contains_two_of_any_letter(&self) -> bool {
    let max = self.sorted_chars.len();

    for (i, c) in self.sorted_chars.iter().enumerate() {
        if i + 1 > max - 1 {
            break;
        }
        if *c != self.sorted_chars[i + 1] {
            continue;
        }

        // might contain three of any letter forward
        if i + 2 < max {
            if *c == self.sorted_chars[i + 2] {
                continue;
            }
        }
        // might contain three of any letter backward
        if i > 0 {
            if *c == self.sorted_chars[i - 1] {
                continue;
            }
        }

        return true;
    }

    false
}
```

จากนั้นสร้างฟังก์ชันสำหรับตรวจสอบตัวอักษรที่ซ้ำกันสามครั้ง หลักการจะคล้ายกับ `contains_two_of_any_letter` แต่ว่าไม่จำเป็นลำดับถัดไปหรือย้อนหลังเพราะโจทย์กำหนดว่าจะมีตัวอักษรซ้ำกันได้มากสุดแค่สามครั้งเท่านั้น

```rust
 fn contains_three_of_any_letter(&self) -> bool {
    let max = self.sorted_chars.len();

    for (i, c) in self.sorted_chars.iter().enumerate() {
        if i + 2 > max - 1 {
            break;
        }
        if *c == self.sorted_chars[i + 1] && *c == self.sorted_chars[i + 2] {
            return true;
        }
    }

    false
}
```

เมื่อได้ฟังก์ชันสำหรับตรวจสอบตัวอักษรซ้ำกันสองและสามครั้งแล้วเอามาประกอบกันที่ `main` จะได้คำตอบสำหรับพาร์ทแรกแล้ว

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut list_of_box_ids: Vec<BoxId> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let box_id = BoxId::from_string(line);
                list_of_box_ids.push(box_id);
            }
        }
    }

    let contains_two: Vec<&BoxId> = list_of_box_ids.iter().filter(|box_id| box_id.contains_two_of_any_letter()).collect();
    let contains_three: Vec<&BoxId> = list_of_box_ids.iter().filter(|box_id| box_id.contains_three_of_any_letter()).collect();
    println!("first part answer is: {}", contains_two.len() * contains_three.len());
}
```

---

ต่อกันที่พาร์ทสองโจทย์ต้องการให้หาว่าหมายเลขกล่องใดบ้างที่ใกล้เคียงกันโดยยกตัวอย่างตามนี้

```
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
```

ซึ่งหมายเลข `fghij` และ `fguij` มีความใกล้เคียงกันมากที่สุดโดยต่างกันแค่หนึ่งตำแหน่งคือ `h` และ `u` ซึ่งถ้าเอาตำแหน่งที่ต่างกันออกจะได้หมายเลขกล่องที่เป็นคำตอบคือ `fgij`

เนื่องจากพาร์ทแรกหมายเลขกล่องที่ได้มาจะถูกเรียงลำดับ ดังนั้นจึงต้องเพิ่มอีกตัวฟิลด์เข้าไปที่ `BoxId` เพื่อเก็บหมายเลขกล่องก่อนที่จะถูกเรียงลำดับ

```rust
#[derive(Debug, Default)]
struct BoxId {
    // for comparing with each other BoxId
    chars: Vec<char>,
    sorted_chars: Vec<char>,

}
```

จากนั้นสร้างฟังก์ชันสำหรับหาว่ามีตัวอักษรที่แตกต่างกันกี่ตำแหน่ง

```rust
/// Return number of different characters between box id `a` and `b`.
fn differ(a: &BoxId, b: &BoxId) -> usize {
    let mut diff: usize = 0;
    for (i, c) in a.chars.iter().enumerate() {
        if *c != b.chars[i] {
            diff += 1;
        }
    }
    diff
}
```

ต่อมาก็เอารายการกล่องทั้งหมดมาวนลูปโดยเริ่มตรวจสอบไปทีละกล่องเป็นคู่ ๆ ซึ่งจะเป็นการวนลูปแบบ n^2 และในลูปจะเรียกใช้ฟังก์ชัน `differ` เพื่อหาว่าหมายเลขกล่องใดบ้างที่มีความแตกต่างที่น้อยที่สุด

```rust
fn find_two_correct_box_ids(list_of_box_ids: &Vec<BoxId>) -> Option<(&BoxId, &BoxId)> {
    for (i, a) in list_of_box_ids.iter().enumerate() {
        for b in list_of_box_ids.iter().skip(i + 1) {
            if differ(a, b) == 1 {
                return Some((a, b));
            }
        }
    }
    None
}
```

พอได้หมายเลขกล่องที่เป็นคำตอบออกมาสองกล่องแล้วก็เอามาอินเตอร์เซกชันเพื่อเอาเฉพาะตัวอักษรที่เหมือนกัน

```rust
/// Return a set of characters are in both of box id `a` and `b`.
fn intersect(a: &BoxId, b: &BoxId) -> Vec<char> {
    let mut chars: Vec<char> = vec![];
    for (i, c) in a.chars.iter().enumerate() {
        if *c == b.chars[i] {
            chars.push(*c);
        }
    }
    chars
}
```

เอาโค้ดทั้งหมดมาประกอบกันก็จะได้คำตอบของพาร์ทที่สองแล้ว

```rust
fn main() {
    ...
    
    if let Some((a, b)) = find_two_correct_box_ids(&list_of_box_ids) {
        println!("second part answer is: {}", String::from_iter(intersect(&a, &b)));
    }
}
```

---
#advent-of-code #rust