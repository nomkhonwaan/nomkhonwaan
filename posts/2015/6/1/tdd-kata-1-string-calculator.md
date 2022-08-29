---
title: TDD Kata 1 - String Calculator
publish_date: 2015-06-01
tags: ['tdd', 'kata', 'go']
---

ในการเขียนโปรแกรมมีแนวคิดนึงที่ยึดหลักการเขียนเทสก่อนลงมืออิมพลิเมนต์ฟังก์ชันจริงหรือที่เรียกว่า Test-driven development (TDD) หลักการคือเริ่มต้นด้วยการเขียนเทส, รันแล้วพัง, แก้ไขโปรแกรม, รันแล้วผ่าน วนอยู่แบบนี้

แต่มันไม่ใช่เรื่องง่ายที่จะจินตนาการว่าเทสควรเป็นอย่างไรเพราะยังไม่ได้อิมพลิเมนต์ฟังก์ชันนั้นจริง ๆ แต่ของแบบนี้สามารถฝึกฝนกันได้ มารู้จักกับการฝึกฝนที่เรียกว่า TDD Kata กัน อนึ่งคำว่า Kata ในภาษาญี่ปุ่นหมายถึงการฝึกฝนซ้ำแล้วซ้ำเล่าจนร่างกายสามารถจดจำได้เอง

> Kata is to learn by repeating over and over again

## สารบัญ 

- [TDD Kata 1 - String Calculator](/2015/6/1/tdd-kata-1-string-calculator)
- [TDD Kata 2 - Bowling Game](/2016/1/28/tdd-kata-2-the-bowling-game)

---

โจทย์ในครั้งนี้คือ [String Calculator](https://osherove.com/tdd-kata-1/) คำแนะนำคือทำไปทีละขั้นอย่าอ่านล่วงหน้าและควรใช้เวลาทำโจทย์ไม่เกิน 15 นาที ถ้าพร้อมแล้วลุย!

1. สร้างฟังก์ชันสำหรับคำนวณสตริงโดยมีอินพุตและเอาท์พุตแบบนี้

```
int Add(string numbers)
```

ฟังก์ชันสามารถรับค่าเป็นตัวเลข​ได้สูงสุดสองตัว โดยแต่คั่นด้วยเครื่องหมายลูกน้ำ (,) และส่งเอาท์พุตเป็นผลรวมของตัวเลขที่รับเข้ามา ตัวอย่างอินพุตเช่น "" หรือ "1" หรือ "1,2" ล้วนยอมรับ ในกรณีที่สตริงเป็นค่าว่างให้ถือว่าเป็น 0

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

มาเขียนเทสกันก่อนโดยเพิ่ม `willPanic` เข้าไปเพื่อให้รู้ว่าเคสไหนบ้างที่ต้องตรวจสอบว่าจะเกิดข้อผิดพลาดได้

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

เริ่มจากแปลง "\n" ให้เป็น "," แบบนี้ ทำให้ไม่ว่าจะส่งตัวคั่นอะไรมาโปรแกรมก็รู้จัก 

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

ตัวอย่างเช่น "//;\n1;2" จะต้องได้ผลลัพธ์เท่ากับ 3 โดยที่ตัวคั่นเป็นเครื่องเซมิโคลอน ";" นอกจากนี้เคสทั้งหมดที่ทำไปข้างต้นจะต้องทำงานได้อยู่เหมือนเดิม

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

เริ่มต้นด้วยการแก้ไขฟังก์ชัน `toNumbers` ให้รับพารามิเตอร์เป็น `numbers` กับ `delimiter` ต่อมาเปลี่ยน `strings.ReplaceAll` กับ `strings.Split` ให้รับตัวแปร `delimiter` แทนที่ลูกน้ำ `,` และสุดท้ายให้ฟังก์ชัน `Add` เรียกใช้งานฟังก์ชัน `parseString` ก่อนส่งค่าต่อให้กับ `toNumbers`

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

จากนั้นรันเทสดูจะพบว่าผ่านหมดแล้วทุกเคส มาดูที่ข้อถัดไปกัน

---

5. ถ้าส่งตัวเลขติดลบเข้ามาให้แสดงข้อผิดพลาด

นอกจากนี้ต้องแสดงตัวเลขติดลบที่เจอในสตริงออกมาด้วยในขณะเดียวกันถ้ามีตัวเลขติดลบมากกว่าหนึ่งให้แสดงทั้งหมดออกมาด้วยเช่นกัน ก่อนอื่นแก้ไขเทสโดยเพิ่ม `panicMessage` สำหรับใช้ตรวจสอบว่ากรณีเกิดข้อผิดพลาดขึ้นจะได้ข้อความตามที่กำหนดไว้หรือไม่

```go
# string_calculator_test.go

func TestAdd(t *testing.T) {
        // Given
        tests := map[string]struct {
                input        string
                expected     int
                willPanic    bool
                panicMessage string // a panic message for assertion
        }{
                ...
                `Should panic with "negative not allowed: -1" message`: {
                        input:        "-1,2",
                        willPanic:    true,
                        panicMessage: "negative not allowed: -1",
                },
                `Should panic with "negative not allowed: -1,-3,-5" message`: {
                        input:        "-1,2,-3,4,-5",
                        willPanic:    true,
                        panicMessage: "negative not allowed: -1,-3,-5",
                },
        }

        // When
        for name, test := range tests {
                t.Run(name, func(t *testing.T) {
                        if test.willPanic {
                                defer func() {
                                        if r := recover(); r == nil {
                                                t.Error("expected panic")

                                                if test.panicMessage != "" && r != test.panicMessage {
                                                        t.Errorf("expected %s but got: %s", test.panicMessage, r)
                                                }
                                        }
                                }()
                        }

                        ...
                })
        }
}
```

รันเทสเพื่อดูผลลัพธ์ซึ่งพังแน่นอน ต่อจากนั้นถึงขั้นตอนการแก้ไขโปรแกรมเพื่อให้รันเทสผ่านดังนี้

```go
# string_calculator.go


func Add(numbers string) int {
        ...
        var sum int
        negatives := make([]int, 0)
        for _, i := range n {
                if i < 0 {
                        negatives = append(negatives, i)
                }
                sum += i
        }

        if len(negatives) > 0 {
                panic(fmt.Sprintf("negative not allowed: %v", negatives))
        }

        ...
}
```

เริ่มจากการตรวจสอบตัวเลขทุกตัวก่อนจะทำการบวกเข้าไปที่ตัวแปร `sum` ถ้าเป็นจำนวติดลบให้เพิ่มเข้าไปอยู่ในลิสต์ของ `negatives` หลังจากนั้นเมื่อจบลูปถ้าตัวแปร `negatives` มีจำนวนมากกว่า 1 ให้สั่ง `panic` และพิมพ์จำนวนตัวเลขติดลบทั้งหมดออกมาในข้อความข้อผิดพลาด เสร็จแล้วทดลองรันอีกทีจะพบว่าตอนนี้เทสผ่านหมดแล้ว ไปดูข้อถัดไปกันดีกว่า

---
6. ตัวเลขที่มีค่ามากกว่า 1,000 ไม่ต้องเอามาคำนวณ

ยกตัวอย่างเช่นถ้ามี 1,001 + 2 ผลลัพธ์ที่ได้จะเท่ากับ 2 เนื่องจากตัวเลข 1,001 ไม่ถูกเอามาคำนวณด้วย ทำการเพิ่มเทสเคสนี้เข้าไปแบบนี้

```go
# string_calculator_test.go


func TestAdd(t *testing.T) {
        // Given
        tests := map[string]struct {
                input        string
                expected     int
                willPanic    bool
                panicMessage string // a panic message for assertion
        }{
                ...
                "Should return 2": {
                        input:    "1001,2",
                        expected: 2,
                },
        }

        ...
}
```

ถ้ารันเทสดูจะพบว่าโปรแกรมรีเทิร์นเป็น 1,003 เนื่องจากยังไม่ได้อิมพลิเมนต์เงื่อนไขตรงนี้เข้าไป จากนั้นแก้ไขฟังก์ชัน `Add` โดยเพิ่มเงื่อนไขตรวจสอบค่าของตัวแปร `i` ก่อนทำการบวกเข้าไปที่ `sum` ทุกครั้งแบบนี้

```go
# string_calculator.go

func Add(numbers string) int {
        ...
        for _, i := range n {
                ...
                if i > 1000 {
                        continue
                }
                sum += i
        }

        ...
}
```

เท่านี้เทสก็ผ่านแล้วไปที่ข้อถัดไปกันเลย

---

7. ตัวคั่นสามารถมีความยาวเท่าไรก็ได้

ตัวอย่างเช่น "//[\**\*]\n1\*\*\*2\*\*\*3" จะต้องได้ผลลัพธ์เท่ากับ 6 เริ่มจากเพิ่มเทสเข้าไปตามนี้

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
                "With *** delimiter, should return 6": {
                        input:    "//***\n1***2***3",
                        expected: 6,
                },
        }

        ...
}
```

เทสนี้จะรันผ่านอยู่แล้วโดยปริยายอันเนื่องจากฟังก์ชัน `parseString` ที่ใช้เป็น regular expression ดังนั้นไปข้อถัดไปเลย

---

8. รองรับตัวคั่นที่หลากหลายและมากกว่าหนึ่งแบบ

จากเดิมที่สามารถระบุตัวคั่นเข้ามาได้เพียงหนึ่งแบบให้แก้ไขเพื่อรองรับการระบุตัวคั่นที่มากกว่าหนึ่งแบบได้ตามรูปแบบนี้

```
//[delim1][delim2]\n
```

ตัวอย่างเช่น "//[\*][%]\n1\*2%3" จะต้องได้ผลลัพธ์เท่ากับ 6 โดยทั้งเครื่องหมายดอกจัน "\*" และเครื่องหมายเปอร์เซ็นต์ "%" สามารถเป็นตัวคั่นได้ทั้งคู่

เริ่มจากเพิ่มเทสเคสนี้เข้าไปก่อนตามนี้

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
                "With multiple delimiters, should return 6": {
                        input:    "//[*][%]\n1*2%3",
                        expected: 6,
                },
        }

        ...
}
```

รันเทสเพื่อดูผลลัพธ์จะพบว่าโปรแกรมพยายามจะแปลง "*" และ "%" ให้เป็นตัวเลขเนื่องจากยังไม่รู้จักรูปแบบตัวคั่นใหม่นี้ ได้เวลาแก้ไขโปรแกรม​โดยครั้งนี้จะแก้ไขที่ฟังก์ชัน `parseString` และเพิ่มฟังก์ชันใหม่ `parseDelimiters` และ `buildReplacer` ตามนี้

```go
# string_calculator.go

// parse delimiters string into slice of string.
func parseDelimiters(delimiters string) []string {
        re := regexp.MustCompile(`\[([^\]\[\r\n]*)\]`)
        matches := re.FindAllString(delimiters, -1)

        // This means the given delimiters matches with the pattern.
        if len(matches) > 0 {
                return matches
        }

        return nil
}

// builder a replacer which is a pair of old-new string
func buildReplacer(delimiters []string) []string {
        replacer := make([]string, 0)
        for _, delimiter := range delimiters {
                replacer = append(replacer,
                        // trim unwanted [ and ]
                        strings.TrimRight(
                          strings.TrimLeft(delimiter, "["), "]",
                        ),
                        ",",
                )
        }
        return replacer
}
```

ที่ฟังก์ชัน `parseDelimiters` จะทำหน้าที่แปลงรูปแบบของตัวคั่นให้เป็น `[]string` เพื่อส่งต่อให้กับฟังก์ชัน `buildReplacer` เพื่อสร้างตัวแปลงสำหรับเปลี่ยนตัวคั่นทั้งหมดที่ส่งมาในสตริงให้เป็นลูกน้ำแทน 

ยกตัวอย่างเช่นถ้าส่งตัวคั่นเข้ามาเป็น "[\*][%]" ที่ฟังก์ชัน `parseDelimiters` จะได้ผลลัพธ์เป็น `[]string{"[*]", "[%]"]}` และเมื่อส่งต่อให้กับฟังก์ชัน `buildReplacer` จะได้ผลลัพธ์ออกมาเป็น `[]string{"*", ",", "%", ","}`

เสร็จแล้วทำการแก้ไขฟังก์ชัน `parseString` เพื่อเรียกใช้งานทั้งสองฟังก์ชันที่สร้างใหม่ตามนี้

```go
# string_calculator.go

func parseString(numbers string) (string, string) {
        ...
        if len(matches) == 3 {
                delimiters := parseDelimiters(matches[1])
                if delimiters == nil {
                        return matches[1], matches[2]
                }
                if len(delimiters) == 1 {
                        return delimiters[1], matches[2]
                }
                // convert all delimiters in the numbers string with comma instead
                return ",", strings.NewReplacer(buildReplacer(delimiters)...).Replace(matches[2])
        }

        ...
}
```

จากเดิมที่ตรวจสอบแค่ `if len(matches) == 3` หรือไม่แล้วรีเทิร์นตัวคั่นกับสตริงตัวเลขออกไปเลย ต้องเพิ่มจังหวะการตรวจสอบว่า `delimiters` มีมากกว่าหนึ่งตัวหรือไม่ ถ้าไม่มีหรือมีแค่ 1 จะทำการรีเทิร์นตัวคั่นตัวแรก (เพราะมีแค่ตัวเดียว) กลับไป 

ถ้าไม่อย่างนั้นจะทำการเปลี่ยนตัวคั่นทั้งหมดที่อยู่ในสตริงตัวเลขให้เป็นลูกน้ำโดยจับจากตัวคั่นที่ส่งมาทั้งหมด ถ้ารันเทส ณ ตอนนี้จะพบว่าผ่านหมดแล้วทุกเคส ไปต่อกันที่ข้อสุดท้าย

---

9. รองรับตัวคั่นที่หลากหลายและมากกว่าหนึ่งแบบและมีความยาวเท่าไรก็ได้

เคสนี้จะคล้ายกับข้อก่อนหน้าที่น่าจะผ่านได้ทันทีเพราะใช้ regular expression ในการหาว่าตัวคั่นอยู่ตรงไหน ก่อนอื่นมาเพิ่มเทสสำหรับข้อนี้ก่อน 

โดยอินพุตจะคล้ายกับข้อก่อนหน้าเพียงแต่เพิ่มความยาวดอกจันและเปอร์เซ็นต์เป็นสามเท่า "//[\*\*\*][%%%]\n1\*\*\*2%%%3" ผลลัพธ์จะต้องเท่ากับ 6 จากนั้นรันเทสเพื่อดูผลลัพธ์ซึ่งควรจะผ่านทั้งหมด

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
                "With *** and %%%, should return 6": {
                        input:    "//[***][%%%]\n1***2%%%3",
                        expected: 6,
                },
        }

        ...
}
```

---

ตัวอย่างโค้ดฉบับเต็มสามารถดูได้ที่ [Go Playground](https://go.dev/play/p/HwEj2buPvM2)