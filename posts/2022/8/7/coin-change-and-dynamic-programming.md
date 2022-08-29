---
title: Coin Change และ Dynamic Programming
publish_date: 2022-08-07
tags: ['leetcode', 'dynamic-programming']
---

เรื่องมีอยู่ว่าได้ลองทำโจทย์ [Coin Change](https://leetcode.com/problems/coin-change/) ทีแรกคิดว่าไม่น่ายากแต่พอส่งคำตอบกลับติดอยู่ที่ `[186, 419, 83, 408]` จะทำอย่างไรให้ใช้เหรียญที่น้อยที่สุดเพื่อแทนค่า `6249`?

## TL;DR

[GitHub](https://github.com/nomkhonwaan/nomkhonwaan/blob/main/leetcode/322_coin_change.rs)

---

แรกสุดใช้ [Greedy Algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm) ในการหาคำตอบซึ่งจะเริ่มจากการใช้เหรียญที่มีมูลค่ามากที่สุดก่อนจากนั้นไล่ไปเรื่อย ๆ จนกว่า `amount` จะเป็นศูนย์หรือไม่มีเหรียญเหลือแล้ว 

ตัวอย่างที่ลองเขียนด้วย Rust

```rust
fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut coins = coins;
    let mut amount = amount;

    if amount < 1 {
        return 0;
    }

    coins.sort_by(|a, b| b.cmp(a));

    let mut number_of_coins = 0i32;

    for coin in coins.into_iter() {
        if coin <= amount {
            // divide an amount with coin value to count number of coins in change
            number_of_coins += amount / coin;
            // reduce an amount value using modulo
            amount = amount % coin;
        }

        if amount == 0 {
            return number_of_coins;
        }
    }

    -1
}
```

ปัญหาของวิธีนี้คือมันใช้แก้โจทย์ได้แค่บางส่วน แต่ปัญหาส่วนใหญ่มันใช้ไม่ได้ยกตัวอย่างเช่น `[2, 3, 6, 7]` ถ้าต้องการเปลี่ยนเหรียญเพื่อแทนค่า `12` จะได้ผลลัพธ์ `3` ซึ่งเกิดจาก

```
7 + 3 + 2
``` 

แต่คำตอบที่ดีที่สุด (optimally) คือ `2` ซึ่งเกิดจาก `6 + 6`

---

เนื่องจากโง่เกินกว่าจะหาวิธีได้เลยยอมอ่านฟอรัมที่เขาถกปัญหากัน มีคนแนะนำว่าให้ใช้หลักการ dynamic programming หรือ DP เลยนึกถึงตอนสมัยเรียน data structure 

_ถ้าย้อนเวลากลับไปได้จะบอกตัวเองสมัยเป็นนักศึกษาว่า "อย่าแอบหลับ" วิชานี้อีก_

[Dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming) ถูกคิดค้นขึ้นในปี 1950s โดย Richard Bellman เพื่อใช้ในการแก้ไขปัญหาที่มีความซับซ้อนด้วยการแตกปัญหาออกเป็นชิ้นย่อย ๆ (sub-problem) และใช้กระบวนการเวียนเกิด (recursion) ในการแก้ไขปัญหาย่อย ๆ ไปทีละขั้นจนมาถึงปัญหาใหญ่

สิ่งที่เกิดขึ้นเมื่อแก้ปัญหาด้วยกระบวนการเวียนเกิดคือปัญหาย่อย ๆ ส่วนมากมันเหมือนกัน ([overlapping sub-problems](https://en.wikipedia.org/wiki/Overlapping_subproblems)) กับกระบวนการหาเส้นทางที่ดีที่สุดในโครงสร้างของปัญหาย่อย  ([optimal substructure](https://en.wikipedia.org/wiki/Optimal_substructure))

ลองแก้โจทย์ด้านบนด้วยกระบวนเวียนเกิดเพื่อคำนวณทุกค่าที่สามารถเกิดขึ้นได้หลังจากลบมูลค่าออกด้วยเหรียญใด ๆ

```rust
fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let max = amount + 1;

    // define a stopping point when an amount is less than one
    if amount < 1 {
        return 0;
    }

    let mut res = max;

    for coin in coins.iter() {
        if *coin <= amount {
            let sub_res = Solution::coin_change(coins.clone(), amount - coin);

            // check if the sub-problem response is not -1 which means there is no result
            if sub_res != -1 {
                res = min(sub_res + 1, res);
            }
        }
    }

    if res > amount {
        return -1;
    }

    res
}
```

จะพบว่าแก้โจทย์​ `[2, 3, 6, 7]` ได้อย่างถูกต้องแล้ว ถ้าเขียนเป็นแผงผังการเวียนเกิดเฉพาะกรณีลบด้วยเหรียญ `2` ได้ประมาณนี้

![Recursion Diagram](https://img.pic.in.th/Dynamic-Programming-Recursion-2.png)

แต่ละกล่องแทนฟังก์ชันที่ทำงานในการเวียนเกิด (sub-problem) กล่องที่สีเหมือนกันหมายความว่าทำงานเหมือนกันได้ผลลัพธ์เดียวกัน (overlapping sub-problems) 

ในการคำนวณชุดข้อมูลที่มีขนาดน้อย ๆ อาจจะไม่ได้รู้สึกถึงความแตกต่างมาก แต่ถ้าเป็นชุดตัวเลขที่มีขนาดใหญ่อย่างเช่นโจทย์ `[186, 419, 83, 408]` จะพบว่าใช้เวลาในการคำนวณนานเกินไป

---

หนึ่งในวิธีที่ใช้เพิ่มประสิทธิภาพคือการจดสิ่งที่เคยคำนวณแล้วเอาไว้ ([memoization](https://en.wikipedia.org/wiki/Memoization)) ลองมาปรับปรุงโปรแกรมเพื่อลดการคำนวณซ้ำด้วย `HashMap` แบบนี้

```rust
fn main() {
    let mut table = HashMap::new();

    // set base value
    table.insert(0,0);

    assert_eq!(20, Solution::coin_change(vec![186, 419, 83, 408], 6249, &mut table));
}

fn coin_change(coins: Vec<i32>, amount: i32, table: &mut HashMap<i32, i32>) -> i32 {
    let max = amount + 1;

    // define a stopping point when an amount is less than one
    if amount < 1 {
        return 0;
    }

    // if already calculated, return it
    if let Some(res) = table.get(&amount) {
        return *res;
    }

    let mut res = max;

    for coin in coins.iter() {
        if *coin <= amount {
            let sub_res = Solution::coin_change(coins.clone(), amount - coin, table);

            // check if the sub-problem response is not -1 which means there is no result
            if sub_res != -1 {
                res = min(sub_res + 1, res);
            }
        }
    }

    if res > amount {
        return -1;
    }

    // memo result
    table.insert(amount, res);
    res
}
```

แม้ว่าจะปรับปรุงกระบวนการให้เร็วขึ้นแล้วแต่มันยังมีวิธีที่ดีกว่าคือการคำนวณด้วย Bottom-Up สังเกตจากแผนภาพด้านบนถ้าแทนวิธีการที่ดีที่สุดในแต่ละมูลค่าจากด้านล่างขึ้นบนด้วยอาเรย์จะได้แบบนี้

```
index: [0,  1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
value: [0, -1, 1, 1, 2, 2, 1, 1, 2, 2,  2,  3,  2]
```

จะพบว่ามันมีรูปแบบเป็น `min(table[amount - coin]) + 1` ยกตัวอย่างเช่นมูลค่า `12` ถ้าเอามาเข้าสูตรตามตำนวนเหรียญ `[2, 3, 6, 7]` จะได้แบบนี้

```
table[12 - 2] = 2 // index 10, value 2
table[12 - 3] = 2 // index 9,  value 2
table[12 - 6] = 1 // index 6,  value 1
table[12 - 7] = 2 // index 5,  value 2

// find the smallest value and plus 1 for the result
min(2, 2, 1, 2) + 1 = 2
```

มาแก้ไขโปรแกรมให้คำนวณด้วยวิธี Bottom-Up แทนเท่านี้ก็สามารถหาคำตอบของโจทย์ได้ทันเวลาแล้ว

```rust
fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    // use amount plus 1 as max value to avoiding overflow
    let max = amount + 1;
    let mut table = vec![max; amount as usize + 1];

    // base value 
    table[0] = 0;

    // inclusive loop until an amount
    for i in 1..amount + 1 {
        for coin in coins.iter() {
            if *coin <= i {
                table[i as usize] = min(table[i as usize], table[(i - coin) as usize] + 1);
            }
        }
    }

    let res = table[amount as usize];
    if res > amount {
        return -1;
    }

    res
}
```