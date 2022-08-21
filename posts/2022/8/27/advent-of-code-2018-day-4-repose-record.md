---
title: Advent of Code 2018 Day 4 - Repose Record
publish_date: 2022-08-27
---

คุณย่องเข้าไปในตู้เสื้อผ้าอีกอันที่อยู่ตรงข้ามกับห้องแล็บผลิตชุดต้นแบบ คุณสามารถแอบเข้าไปข้างในได้และพบว่ามีปัญหาเกิดขึ้นกับชุดสูท คุณต้องการแก้ไขมันแต่จะทำอย่างไรในเมื่อมียามเฝ้าอยู่ด้านนอกห้องเล็บ

## สารบัญ

- [Advent of Code 2018 Day 1 - Chronal Calibration](/2022/7/6/advent-of-code-2018-day-1-chronal-calibration)
- [Advent of Code 2018 Day 2 - Inventory Management System](/2022/7/28/advent-of-code-2018-day-2-inventory-management-system)
- [Advent of Code 2018 Day 3 - No Matter How You Slice it](/2022/8/5/advent-of-code-2018-day-3-no-matter-how-you-slice-it)
- [Advent of Code 2018 Day 4 - Repose Record](/2022/8/27/advent-of-code-2018-day-4-repose-record)

## TL;DR

[GitHub](https://github.com/nomkhonwaan/nomkhonwaan/blob/main/advent-of-code/2018/day_4_repose_record.rs)

---

ในขณะที่ค้าหาสิ่งที่อาจช่วยได้ในตู้เสื้อผ้า คุณพบว่ามีใครบางคนเคยแอบเข้าไปด้านในมาก่อนและคอยสอดส่องพวกยามที่เข้าเวรในช่วงเที่ยงคืนตลอดสองสามเดือนที่ผ่านมา! 

พวกเอลฟ์คิดว่ายามเพียงคนเดียวก็เพียงพอสำหรับการเฝ้ากะกลางคืน ดูเหมือนว่าคุณพอจะรู้แล้วว่าควรแอบเข้าไปตอนไหนจากข้อมูลการหลับหรือตื่นขณะเฝ้ายาม

---

โจทย์ให้รายงานการเฝ้ายามช่วงระหว่างเวลา 00:00 ถึง 00:59 มาแบบนี้ โดยที่ตัวเวลาจะอยู่ในรูปแบบของ `year-month-day hour:minute` 

```
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
```

จากตัวอย่างข้อมูลที่ได้มาสามารถแสดงในรูปแผนภาพโดยเรียงจากวันที่และรหัสของยามที่เฝ้า ซึ่งนาทีที่ยามหลับจะแทนที่ด้วยสัญลักษณ์​ `#`

```
Date   ID   Minute
            000000000011111111112222222222333333333344444444445555555555
            012345678901234567890123456789012345678901234567890123456789
11-01  #10  .....####################.....#########################.....
11-02  #99  ........................................##########..........
11-03  #10  ........................#####...............................
11-04  #99  ....................................##########..............
11-05  #99  .............................................##########.....
```

โจทย์ต้องการให้หาว่ายามคนไหนที่ใช้เวลานอนหลับมากที่สุด จากนั้นให้เอาหมายเลขไอดีของยามคูณกับนาทีที่ชอบหลับบ่อยที่สุดจะได้ผลลัพธ์ของคำตอบในพาร์ทแรก

เริ่มต้นเหมือนทุกครั้งด้วยการดาวน์โหลดอินพุตไฟล์เข้ามาในโปรแกรมและแปลงให้เป็น `Record` ซึ่งจะเก็บข้อมูลอยู่สองอย่างคือ `date_time` จะถูกแปลงเป็น [`NaiveDateTime`](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html) ซึ่งจะเอามาใช้เพื่อเรียงลำดับรายการตามเวลา `info` จะเป็นส่วนของข้อมูลที่เหลือหลังจากตัดเอาเวลาออกไปใช้สำหรับระบุประเภทของรายการ

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let mut records: Vec<Record> = vec![];

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let record = Record::from_string(line);
                records.push(record);
            }
        }
    }
}

/// Contain each line of the record.
#[derive(Debug, Default)]
struct Record {
    date_time: NaiveDateTime,
    info: String,
}

impl Record {
    fn from_string(s: String) -> Self {
        let re = Regex::new(r"\[(.+)]\s(.+)$").unwrap();
        let captures = re.captures(&s).unwrap();
        let date_time = captures.get(1).map(|m| m.as_str().to_string()).unwrap();

        Record {
            date_time: NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M").unwrap(),
            info: captures.get(2).map(|m| m.as_str().to_string()).unwrap(),
        }
    }
}

fn read_lines<P: AsRef<path::Path>>(path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
```

หลังจากอ่านข้อมูลจนครบทั้งไฟล์แล้วนำมาจัดเรียงใหม่ตามเวลา `date_time` จากน้อยไปมากด้วยฟังก์ชัน `sort_by` แบบนี้

```rust
fn main() {
    ...

    if let Ok(lines) = read_lines(input) {
        for line in lines {
            if let Ok(line) = line {
                let record = Record::from_string(line);
                records.push(record);
            }
        }
    }

    records.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    ...
}
```
 
มาถึงส่วนสำคัญของโจทย์วันนี้นั่นก็คือการแปลงข้อมูลที่ได้มาให้อยู่ในรูปของที่สามารถนำมาคำนวณได้ จากตัวอย่างแผนภาพด้านบนแสดงให้เห็นว่ายามหนึ่งคนจะมีวันที่เฝ้ายามมากกว่าหนึ่งวันได้ดังนั้นข้อมูลของยามจะเก็บอยู่ในรูปแบบ tuple ประกอบไปด้วยหมายเลขไอดีของยามและตารางการทำงาน

```rust
let guard: (String, HashMap<String, Vec<u32>>);
//          |- contain guard id
//                  |- contain guard's duties
//                          |- contain date-time of the duty
//                                  |- list of asleep/awake time

let list_of_guards: Vec<(String, HashMap<String, Vec<u32>>)>;
```

ก่อนอื่นเพิ่มฟังก์ชันสำหรับใช้ตรวจสอบว่าเป็น `begin_record`, `falls_asleep` และ `wakes_up` ที่ `Record` สำหรับใช้เช็คค่าแต่ละบรรทัด

```rust
impl Record {
    ...

    fn begin_record(&self) -> bool {
        self.info.contains("begins")
    }

    fn falls_asleep(&self) -> bool {
        self.info.contains("falls asleep")
    }

    fn wakes_up(&self) -> bool {
        self.info.contains("wakes up")
    }
}

```

จากนั้นวนลูป `records` เพื่ออ่านค่าทีละบรรทัดแล้วเอามาเก็บที่ `list_of_guards` แบบนี้

```rust
let mut duties: HashMap<String, Vec<u32>> = HashMap::new();
let mut guard_id = records.get(0).unwrap().get_guard_id();
let mut begin_sleep_time: Option<NaiveDateTime> = None;
let mut midnight_hour = vec![0; 60];

// skip the first record since it has been recorded at the init step
for record in records.iter().skip(1) {
    if record.begin_record() {
        // store previous record
        if let Some(t) = begin_sleep_time {
            duties.insert(t.format("%Y-%m-%d").to_string(), midnight_hour);
            list_of_guards.insert(guard_id.clone(), duties);
        }

        // init a new record
        guard_id = record.get_guard_id();
        // try to get the previous duties data,
        // will create a new one if not exists
        duties = match list_of_guards.get(&guard_id) {
            Some(duties) => duties.clone(),
            _ => HashMap::new(),
        };
        begin_sleep_time = None;
        midnight_hour = vec![0; 60];
    }

    if record.falls_asleep() {
        begin_sleep_time = Some(record.date_time);
    }

    if record.wakes_up() {
        let end_sleep_time = record.date_time;
        if let Some(mut time) = begin_sleep_time {
            while time < end_sleep_time {
                let i = time.format("%M").to_string().parse::<usize>().unwrap();
                midnight_hour[i] = 1;
                time = time + Duration::minutes(1);
            }
        }
    }
}

// store the latest record if not empty
if let Some(t) = begin_sleep_time {
    duties.insert(t.format("%Y-%m-%d").to_string(), midnight_hour);
    list_of_guards.insert(guard_id.clone(), duties);
}
```

เนื่องจากบรรทัดแรกจะขึ้นต้นด้วย `Guard #XX begins shift` เสมอ ดังนั้นจะเอาค่าของบรรทัดแรกสุดมาเป็นค่าเริ่มต้นและข้ามบรรทัดแรกสุดตอนวนลูปด้วย 

```rust
records.iter().skip(1)
```

(TBC) Explain the accumulate all duties function

```rust
fn accumulate_all_duties(duties: &HashMap<String, Vec<u32>>) -> Vec<u32> {
    let mut accumulated: Vec<u32> = vec![0; 60];
    for duty in duties.iter() {
        for (i, &v) in duty.1.iter().enumerate() {
            if v > 0 {
                accumulated[i] += 1;
            }
        }
    }
    accumulated
}
```