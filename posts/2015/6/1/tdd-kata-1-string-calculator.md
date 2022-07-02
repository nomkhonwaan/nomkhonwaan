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
                input    string
                expected int
        }{
                "Should return 0": {
                        input:    "",
                        expected: 0,
                },
                "Should return 1": {
                        input:    "1",
                        expected: 1,
                },
                "Should return 3": {
                        input:    "1,2",
                        expected: 3,
                },
        }

        // When
        for name, test := range tests {
                t.Run(name, func(t *testing.T) {
                        result := Add(test.input)
                        if result != test.expected {
                                t.Errorf("expected %d but got: %d", test.expected, result)
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

เมื่อรันเทสจะพบว่า _พัง_ ไป 2 ใน 3 เคสซึ่งนี่คือความตั้งใจอยู่แล้ว ลองมาแก้ไขให้มันทำงานได้ถูกต้องตามหลักการ TDD กัน

```go
# string_calculator.go

import (
        "strings"
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

มีการสร้างฟังก์ชันใหม่ `toNumbers` สำหรับทำหน้าที่แปลงสตริงเป็น `[]int` และฟังก์ชัน `Add` ให้ทำหน้าที่รวมผลลัพธ์ ตอนนี้ผลเทสน่าจะผ่านหมดทุกข้อแล้ว ไปดูเงื่อนไขถัดไปกันต่อ

---

2. ทำให้ฟังก์ชัน `Add` สามารถคำนวณผลลัพธ์จากตัวเลขใด ๆ ที่ส่งเข้ามาได้

อันดับแรกต้องเพิ่มเทสเข้าไปก่อนแบบนี้

```go
# string_calculator_test.go

func TestAdd(t *testing.T) {
        // Given
        tests := map[string]struct{
                input    string
                expected int
        }{
                ...
                "Should return -1": {
                        input:    "-2,1",
                        expected: -1,
                },
                "Should return 90": {
                        input:    "89,1",
                        expected: 90,
                },
        }

        ...
}
```

เสร็จแล้วก็รันเทสเพื่อดูว่าต้องแก้ไรฟังก์ชันเพิ่มไหม ก็จะพบว่าเทสผ่านอยู่แล้วทุกข้อ เนื่องจากฟังก์ชัน `toNumbers` มีการแปลงสตริงเป็นตัวเลขได้ตามเงื่อนไขข้อนี้อยู่แล้ว

---

3. ทำให้ฟังก์ชัน `Add` รองรับเครื่องหมายขึ้นบรรทัดใหม่ (เพิ่มเติมจากลูกน้ำ)

กรณีที่อินพุตเป็น "1\n2,3" จะได้ผลลัพธ์เป็น 6 แต่กลับกันถ้าอินพุตเป็น "1,\n" ให้แสดงข้อผิดพลาดแทน แต่เนื่องจากฟังก์ชัน `Add` ไม่ได้มีการรีเทิร์นข้อผิดพลาดกลับ ดังนั้นในข้อที่ต้องแสดงข้อผิดพลาดจะเป็นการสั่ง `panic` ใน Go แทน 

มาเขียนเทสกันก่อนโดยเพิ่ม `willPanic` เข้าไปเพื่อให้รู้ว่าเคสไหนบ้างที่ต้องเช็คว่าจะเกิดข้อผิดพลาดได้

```go
# string_calculator_test.go

func TestAdd(t *testing.T) {
        // Given
        tests := map[string]struct{
                input     string
                expected  int
                willPanic bool
        }{
                ...
                "Should return 6": {
                        input:    "1\n2,3",
                        expected: 6,
                },
                "Should panic": {
                        input:     "1,\n",
                        willPanic: true
                },
        }

        // When
        for name, test := range tests {
                t.Run(name, func(t *testing.T) {
                        if test.willPanic {
                                defer func() {
                                        if r := recover(); r == nil {
                                                t.Error("expected panic")
                                        }
                                }()
                        }

                        result := Add(test.input)
                        if result != test.expected {
                                t.Errorf("expected %d but got: %d", test.expected, result)
                        }
                })
        }
}
```

รันเทสเพื่อดูว่าผลลัพธ์จะเป็นอย่างไร ก็พบว่า 2 เคสที่เพิ่มเข้าไปพังทันที มาแก้ไขฟังก์ชัน `Add` และ `toNumbers` เพื่อให้เทสผ่านดีกว่า

เริ่มจากแปลง `\n` ให้เป็น `,` แบบนี้ ทำให้ไม่ว่าจะส่งตัวคั่นอะไรมาโปรแกรมก็รู้จัก 

```go
# string_calculator.go

// convert numbers string into slice of integer.
func toNumbers(numbers string) []int {
	numbers = strings.ReplaceAll(numbers, "\n", ",")
	n := strings.Split(numbers, ",")
	m := make([]int, 0)

	...
}
```

หลังจากแก้ไขตรงนี้ลองกลับไปรันเทสดูจะพบว่าเคสที่พังในตอนแรกนั้นผ่านแล้ว 1 เคส เหลืออีกเคสที่ต้องแสดงผลข้อผิดพลาดกรณีที่ส่งสตริงเข้ามาเป็น "1,\n"

เหมือนเดิมเราจะแก้ไขกันที่ฟังก์ชัน `toNumbers` เนื่องจากกระบวนการอ่านสตริงแล้วเปลี่ยนเป็น `[]int` ทำที่ฟังก์ชันนี้

ถ้ายังจำได้จะมีจังหวะนึงที่เรียกใช้ `strconv.Atoi` ซึ่งตัวฟังก์ชันนี้จะรีเทิร์นข้อผิดพลาดกลับมาถ้าไม่สามารถแปลงค่าเป็นตัวเลขได้ เดิมเราแค่ปล่อยผ่านแล้ววนลูปต่อ ครั้งนี้เราจะเปลี่ยนให้มัน `panic` แทนแบบนี้

```go
# string_calculator.go

func toNumbers(numbers string) []int {
        ...

        for _, i := range n {
                j, err := strconv.Atoi(i)
                if err != nil {
                        panic(err)
                }
                m = append(m, j)
        }

        ...
}
```

ทีนี้ลองรันเทสดูอีกรอบจะพบว่าผ่านหมดแล้วทุกเคส ไปที่ข้อถัดไปกัน

---

4. รองรับตัวคั่นที่หลากหลายมากขึ้น

จากเดิมที่โปรแกรมรองรับตัวคั่นอยู่สองแบบคือลูกน้ำกับเครื่องหมายขึ้นบรรทัดใหม่ ผู้ใช้สามารถส่งรูปแบบของสตริงที่สามารถระบุตัวคั่นอื่น ๆ เข้ามาได้ในรูปแบบนี้

```
//[delimiter]\n[numbers...]
```

ตัวอย่างเช่น `//;\n1;2` จะต้องได้ผลลัพธ์เท่ากับ 3 โดยที่ตัวคั่นเป็นเครื่องเซมิโคลอน `;` นอกจากนี้เคสทั้งหมดที่ทำไปข้างต้นจะต้องทำงานได้อยู่เหมือนเดิม

เริ่มจากแก้ไขเทสกันก่อน

```go
# string_calculator_test.go

func TestAdd(t *testing.T) {
        // Given
        tests := map[string]struct{
                input     string
                expected  int
                willPanic bool
        }{
                ...
                "With semicolon, should return 3": {
                        input:    "//;\n1;2",
                        expected: 3,
                },
        }

        ...
}
```

แน่นอนว่าพอรันเทสแล้วพังแน่นอนแบบนี้

```
--- FAIL: TestAdd (0.00s)
    --- FAIL: TestAdd/With_semicolon,_should_return_3 (0.00s)
panic: strconv.Atoi: parsing "//;": invalid syntax [recovered]
	panic: strconv.Atoi: parsing "//;": invalid syntax
```

มาแก้ไขฟังก์ชันให้รองรับการแปลงรูปแบบสตริงเป็นตัวคั่นโดยเพิ่มฟังก์ชัน `parseString` อันนี้เข้าไป

```go
# string_calculator.go

import "regexp"

// parse numbers string and return both delimiter (if exist or default comma) and numbers string.
func parseString(numbers string) (string, string) {
        re := regexp.MustCompile(`^//(.+)\n(.+)`)
        matches := re.FindStringSubmatch(numbers)

        // This means the given numbers string matches with the pattern,
        // then set the delimiter with the given string.
        if len(matches) == 3 {
                return matches[1], matches[2]
        }

        // There is no matching pattern, so just return the given numbers string
        // with the default delimiter.
        return ",", numbers
}
```

จากนั้นแก้ไขฟังก์ชัน `toNumbers` ให้รับพารามิเตอร์เป็น `numbers` กับ `delimiter` และเปลี่ยนตรง `strings.ReplaceAll` กับ `strings.Split` ให้ใช้ตัวแปร `delimiter` แทนที่จะเป็น `,` และให้ฟังก์ชัน `Add` เรียกใช้งานฟังก์ชัน `parseString` ก่อนส่งค่าให้กับ `toNumbers` แบบนี้

```go
# string_calculator.go

func Add(numbers string) int {
        ...

        n := toNumbers(parseString(numbers))

        ...
}

func toNumbers(delimeter, numbers string) []int {
        numbers = strings.ReplaceAll(numbers, "\n", delimiter)
        n := strings.Split(numbers, delimiter)
        ...
}
```

ตอนนี้ถ้ารันเทสดูจะพบว่าผ่านหมดแล้วทุกเคส มาดูที่ข้อถัดไปกัน

---

5. ถ้าส่งตัวเลขติดลบเข้ามาให้