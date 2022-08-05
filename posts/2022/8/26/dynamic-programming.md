---
title: Dynamic Programming
publish_date: 2022-08-26
---

เรื่องมีอยู่ว่าได้ลองทำโจทย์ [Coin Change](https://leetcode.com/problems/coin-change/) แต่มันติดอยู่ที่ถ้ามีเหรียญ `[186, 419, 83, 408]` จะเปลี่ยนจำนวน `6249` ให้เป็นเหรียญด้วยจำนวนน้อยที่สุดได้ยังไง?

---

เริ่มต้นตอนแรกด้วยการใช้ [Greedy Algorithm](https://en.wikipedia.org/wiki/Greedy_algorithm) ซึ่งมันจะเอาเหรียญที่มีมูลค่ามากที่สุดมาแลกจากนั้นจึงใช้เหรียญมูลค่ารองลงมา

ตัวอย่างของ Greedy Algorithm ที่เขียนด้วย Rust

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
            // devide an amount with coin value to count number of coins in change
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

ซึ่งถ้าเอามาใช้กับเคสเหล่านี้จะยังสามารถใช้หาคำตอบที่ถูกต้องได้อยู่

- `coins: [1, 2, 5], amount: 11, expected: 3`
- `coins: [2], amount: 3, expected: -1`
- `coins: [1], amount: 0, expected: 0`

แต่ปัญหาคือกับเคสส่วนใหญ่มันใช้ไม่ได้ยกตัวอย่างเช่น

- `coins: [20, 12, 1], amount: 48, expected: 4` ถ้าใช้ Greedy Algorithm จะได้ `10` เกิดจาก `20 + 20 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1` แทนที่จะเป็น `12 + 12 + 12 + 12` ซึ่งใช้แค่สี่เหรียญ
- `coins: [2, 3, 6, 7], amount: 12, expected: 2` ถ้าใช้ Greedy Algorithm จะได้ `3` เกิดจาก `7 + 3 + 2` แทนที่จะเป็น `6 + 6` ซึ่งใช้แค่สองเหรียญ

---


คือพยายามจะใ