---
title: Advent of Code 2018 Day 3 - No Matter How You Slice it
publish_date: 2022-08-05
---

ในที่สุดบรรดาเอลฟ์ก็สามารถสร้างตัวอย่างชุดซานต้าต่อได้ (ต้องขอบคุณใครบางคนที่เขียนหมายเลขกล่องที่ถูกต้องไว้บนกำแพงเมื่อคืน) แต่เรื่องราวยังคงไม่จบพวกเขาเถียงกันว่าจะตัดผ้าอย่างไรดี

## สารบัญ

- [Advent of Code 2018 Day 1 - Chronal Calibration](/2022/7/6/advent-of-code-2018-day-1-chronal-calibration)
- [Advent of Code 2018 Day 2 - Inventory Management System](/2022/7/28/advent-of-code-2018-day-2-inventory-management-system)
- [Advent of Code 2018 Day 3 - No Matter How You Slice it](/2022/8/5/advent-of-code-2018-day-3-no-matter-how-you-slice-it)

## TL;DR

[GitHub](https://github.com/nomkhonwaan/nomkhonwaan/blob/main/advent-of-code/2018/day_3_no_matter_how_you_slice_it.rs)

---

ผ้าที่พวกเขาต้องการจะตัดนั้นมีขนาดค่อนข้างใหญ่เป็นรูปทรงสี่เหลี่ยมจตุรัสมีขนาดแต่ละด้านยาว 1,000 นิ้ว (ประมาณ 25.4 เมตร) 

เอลฟ์แต่ละคนต่างก็อ้างว่าบริเวณผ้าของตนเหมาะที่จะตัดเป็นชุดของซานต้ากันทั้งนั้น

---

มาถึงส่วนของโจทย์ในวันนี้

เอลฟ์แต่ละคนจะมีหมายเลขที่ใช้สำหรับอ้างอิงบริเวณผ้าของตัวเองอยู่ในรูปแบบ `#123 @ 3,2: 5x4` โดยที่ `123` คือหมายเลขไอดี, `3` คือความห่างจากขอบซ้ายของผ้าสามนิ้ว, `2` คือความห่างจากขอบบนของผ้าสองนิ้ว, `5` คือความกว้างของบริเวณผ้า, `4` คือความสูงของบริเวณผ้า สามารถแสดงเป็นแผนภาพได้ดังนี้

โดยส่วนที่เป็น `#` คือบริเวณที่ถูกอ้างสิทธิ์อยู่และ `.` คือส่วนที่ยังไม่มีเจ้าของ

```
...........
...........
...#####...
...#####...
...#####...
...#####...
...........
...........
...........
```

ปัญหาก็คือบริเวณผ้าของแต่ละเอลฟ์มันทับซ้อนกันอยู่ยกตัวอย่างรายการอ้างสิทธิ์ด้านล่าง

```
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
```

เมื่อแสดงเป็นแผนภาพจะเห็นส่วนที่ทับซ้อนกันระหว่างหมายเลข `1` และ `2` แสดงเป็นตัว `X` แบบนี้

```
........
...2222.
...2222.
.11XX22.
.11XX22.
.111133.
.111133.
........
```

โจทย์ต้องการให้หาว่ามีพื้นที่ทับซ้อนกันมากกว่าหนึ่งครั้งทั้งหมดเท่าไร เริ่มต้นเหมือนกันกับวันก่อนหน้าด้วยการดาวน์โหลดอินพุต "input.txt" 

จากนั้นใช้ฟังก์ชันเดิมเพื่ออ่านทีละบรรทัด จากนั้นสร้าง `Rectangle` สำหรับใช้เก็บค่าความห่างจากขอบทั้งบนและล่างกับความกว้างและความสูงแบบนี้

ที่ `from_string` จะใช้ Regular Expression เพื่อแปลงค่าที่ส่งเข้ามา สามารถใช้เว็บ [regex101](https://regex101.com/) เพื่อดูการจับคู่กับสตริงได้


```rust
/// Contain each rectangle data.
#[derive(Default)]
struct Rectangle {
    id: String,
    left_edge: i32,
    top_edge: i32,
    wide: i32,
    tall: i32,
}

impl Rectangle {
    fn from_string(s: String) -> Self {
        let re = Regex::new(r"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
        let captures = re.captures(&s).unwrap();

        Rectangle {
            id: captures.get(1).map(|m| m.as_str().to_string()).unwrap(),
            left_edge: captures.get(2).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            top_edge: captures.get(3).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            wide: captures.get(4).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
            tall: captures.get(5).map(|m| m.as_str().parse::<i32>().unwrap()).unwrap(),
        }
    }
}
```

กลับมาที่ `main` จะคล้ายเดิมคือทำการวนลูปทีละบรรทัดและแปลงค่าสตริงเป็น `Rectangle`

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut list_of_rectangles: Vec<Rectangle> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let rectangle = Rectangle::from_string(line);
                list_of_rectangles.push(rectangle);
            }
        }
    }
}
```

จากนั้นทำการสร้างจุดจากตำแหน่งของแต่ละสี่เหลี่ยมโดยเริ่มจากด้านซ้ายบนไปจนถึงด้านขวาล่าง ยกตัวอย่างเช่นถ้าสี่เหลี่ยมอยู่ห่างจากขอบบน 2 และขอบซ้าย 3 โดยมีความกว้าง 4 และความสูง 5 

```
..........
..........
...xxxx...
...xxxx...
...xxxx...
...xxxx...
...xxxx...
..........
..........
```

จะได้ตำแหน่งทั้งหมดตามนี้

```
(3,2), (4,2), (5,2), (6,2)
(3,3), (4,3), (5,3), (6,3)
(3,4), (4,4), (5,4), (6,4)
(3,5), (4,5), (5,5), (6,5)
(3,6), (4,6), (5,6), (6,6)
```

ตัวฟังก์ชันไม่ซับซ้อนอะไรใช้การวนลูปจากขอบด้านซ้ายไปจนถึงด้านขวาและซ้อนด้วยลูปจากขอบด้านบนไปจนถึงด้านล่าง

```rust

impl Rectangle {
    fn from_string(s: String) -> Self {
        ...
    }

    fn to_points(&self) -> Vec<(i32, i32)> {
        let mut points: Vec<(i32, i32)> = vec![];
        for i in self.left_edge..self.left_edge + self.wide {
            for j in self.top_edge..self.top_edge + self.tall {
                points.push((i, j));
            }
        }
        points
    }
}
```

กลับมาที่ `main` ทำการวนลูปและแปลงค่าเป็น `Vec<(i32, i32)>` จากนั้นเอาไปใส่ `HashMap<(i32, i32), i32>` เพื่อที่จะนับว่าตำแหน่งนี้โดนอ้างถึงกี่ครั้ง และสุดท้ายทำการนับตำแหน่งที่ถูกอ้างถึงมากกว่าหนึ่งครั้งก็จะได้คำตอบของพาร์ทแรกแล้ว

```rust
let counted_points: HashMap<(i32, i32), i32> = list_of_rectangles
    .clone()
    .iter()
    // at this point, the rectangle will become vector of points
    //
    // [(x0,y0), (x1,y1), ..., (xn, yn)]
    .flat_map(|rectangle| rectangle.to_points())
    // at this point, the vector of points will become a hashmap like this
    //
    // [
    //   (x0,y0): 1,
    //   (x1,y1): 2,
    //   ...
    //   (xn,yn): 1,
    // ]
    .fold(HashMap::new(), |mut result, pair| {
        // insert pair to the hash map and increase count by 1
        result.insert(pair, match result.get(&pair) {
            Some(count) => count + 1,
            _ => 1,
        });
        result
    });

let collided_points: HashMap<(i32, i32), i32> = counted_points
    .clone()
    .into_iter()
    .filter(|(_, v)| *v > 1) // to filter only points are collided more than 1
    .collect();

println!("first part answer is: {}", collided_points.len());
```

---

พาร์ทที่สองโจทย์ต้องการให้หาว่าสี่เหลี่ยมใดที่ไม่ถูกอ้างอิงเลย (เช่นสี่เหลี่ยมหมายเลข 3 จากตัวอย่างข้างบน)

ทำคล้าย ๆ กันโดยแทนที่จะกรองเอาจุดที่ถูกอ้างอิงมากกว่าหนึ่งครั้งเปลี่ยนเป็นเอาเฉพาะจุดที่ถูกอ้างอิงเพียงครั้งเดียวเท่านั้น

```rust
let non_collided_points: Vec<(i32, i32)> = counted_points
    .clone()
    .into_iter()
    .filter(|(_, v)| *v == 1) // to filter only points are not counted more than 1
    // at this point, the hashmap will become a vector of points
    //
    // [(x0,y0), (x1,y1), ..., (xn,yx)]
    .map(|(k, _)| k)
    .collect();
```

จากนั้นก็สั่งวนลูปสี่เหลี่ยมทั้งหมดเพื่อเอาจุดมาเช็คว่าสี่เหลี่ยมไหนที่จุดทั้งหมดอยู่ในรายการของ `non_collided_points` และเมื่อเจอก็สั่งให้หยุดการทำงานจากลูปและพิมพ์หมายเลขสี่เหลี่ยมเพื่อแสดงคำตอบของพาร์ทที่สอง

```rust
for rectangle in list_of_rectangles.into_iter() {
    let yes = rectangle.to_points()
        .iter()
        .all(|point| {
            non_collided_points.contains(point)
        });
    if yes {
        println!("second part answer is: {}", &rectangle.id);
        break;
    }
}
```

---
#advent-of-code #rust