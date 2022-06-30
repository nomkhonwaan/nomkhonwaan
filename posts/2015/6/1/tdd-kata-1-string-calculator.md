---
title: TDD Kata 1 - String Calculator
publish_date: 2015-06-01
---

ในการเขียนโปรแกรมมีแนวคิดนึงที่ยึดหลักการเขียนเทสก่อนลงมืออิมพลิเมนต์ฟังก์ชันจริงหรือที่เรียกว่า Test-driven development (TDD) หลักการคือเริ่มต้นด้วยการเขียนเทส, รันแล้วพัง, แก้ไขโปรแกรม, รันแล้วผ่าน วนอยู่แบบนี้

แต่มันไม่ใช่เรื่องง่ายที่จะจินตนาการว่าเทสควรเป็นอย่างไรเพราะยังไม่ได้อิมพลิเมนต์ฟังก์ชันนั้นจริง ๆ แต่ของแบบนี้สามารถฝึกฝนกันได้ มารู้จักกับการฝึกฝนที่เรียกว่า TDD Kata กัน อนึ่งคำว่า Kata ในภาษาญี่ปุ่นหมายถึงการฝึกฝนซ้ำแล้วซ้ำเล่าจนร่างกายสามารถจดจำได้เอง

> Kata is to learn by repeating over and over again

## สารบัญ 

- TDD Kata 1 - String Calculator
- [TDD Kata 2 - Bowling Game](/2016/1/28/tdd-kata-2-bowling-game)

---

โจทย์ในครั้งนี้คือ [String Calculator](https://osherove.com/tdd-kata-1/) คำแนะนำคือทำไปทีละขั้นอย่าอ่านล่วงหน้าและควรใช้เวลาทำโจทย์ไม่เกิน 15 นาที ถ้าพร้อมแล้วลุย!

1. สร้างฟังก์ชันสำหรับคำนวณสตริงโดยมีอินพุตและเอาท์พุตแบบนี้

```
int Add(string numbers)
```

ฟังก์ชันสามารถรับค่าเป็นตัวเลข​ได้สูงสุดสองตัว โดยแต่คั่นด้วยเครื่องหมายลูกน้ำ (`,`) และส่งเอาท์พุตเป็นผลรวมของตัวเลขที่รับเข้ามา ตัวอย่างอินพุตเช่น "" หรือ "1" หรือ "1,2" ล้วนยอมรับ ในกรณีที่สตริงเป็นค่าว่างให้ถือว่าเป็น 0

เริ่มต้นจากการเขียนเทสด้วย

```go
# string_calculator_test.go
func TestAdd(t *testing.T) {
        // Given
        tests := map[string]struct{
                input string
                expected int
        }{
                "Should return 0": {
                        input: "",
                        expected: 0,
                },
                "Should return 1": {
                        input: "1",
                        expected: 1,
                },
                "Should return 3": {
                        input: "1,2",
                        expected: 3,
                }
        }

        // When
        for name, test := range tests {
                t.Run(name, func(t *testing.T) {
                        result := Add(test.input)
                        if result != test.expected {
                                t.Errorf("expected %s but got: %s", test.expected, result)
                        }
                })
        }
}
```

และแน่นอนว่าเราต้องสร้างฟังก์ชัน `Add` ขึ้นมาด้วยโดยจะยังไม่มีการอิมพลิเมนต์อะไรนอกจากส่งค่า "0" กลับไป

```go
# string_calculator.go
func Add(numbers string) int {
        return 0
}
```

เสร็จแล้วมารันเทสก็จะพบว่า_พัง_ไปสองในสามเคส ซึ่งแน่นอนว่านี้คือความตั้งใจอยู่แล้ว ลองมาแก้ไขให้มันทำงานได้ถูกต้องตามหลักการ TDD กัน

```go
# string_calculator.go

import (
        "string"
        "strconv"
)

func Add(numbers string) int {
        if numbers == "" {
                return 0
        }
      
        n := toNumbers(numbers)
        var sum int 
        for _, i := range n {
                sum += i
        }

        return sum 
}

// convert numbers string into slice of integer.
func toNumbers(numbers string) []int {
        n := strings.Split(numbers, ",")
        m := make([]int, 0)
        
        for _, i := range n {
                j, err := strconv.Atoi(i)
                if err != nil {
                        continue
                }
                m = append(m, j)
        }

        return m
}
```

เอาละมีการสร้างฟังก์ชันใหม่ `toNumbers` สำหรับทำหน้าที่แปลงสตริงเป็น `[]int` และฟังก์ชัน `Add` ก็ให้ทำหน้าที่รวมผลลัพธ์ ตอนนี้ถ้ารันเทสทุกเคสน่าจะผ่านหมดแล้ว ไปดูเงื่อนไขถัดไปกัน

2. 