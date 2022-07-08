---
title: TDD Kata 2 - The Bowling Game
publish_date: 2016-01-28
---

กลับมาพบกันอีกครั้งกับ TDD Kata โจทย์ในวันนี้คือ [The Bowling Game Kata](http://butunclebob.com/ArticleS.UncleBob.TheBowlingGameKata) รายละเอียดของโจทย์จะอยู่ในสไลด์ที่อยู่ในบทความอีกที ถ้าพร้อมแล้วไปลุยกันดีกว่า


## สารบัญ 

- [TDD Kata 1 - String Calculator](/2015/6/1/tdd-kata-1-string-calculator)
- TDD Kata 2 - Bowling Game

---

ตัวโจทย์ต้องการสร้างโปรแกรมสำหรับคำนวณคะแนนที่ได้จากการโยนบอลโดยมีตารางคะแนน (เฟรม)​ แบ่งเป็น 10 ช่อง โดยที่แต่ละช่องจะบอกถึงคะแนนที่ผู้เล่นทำได้ในการโยนแต่ละครั้ง 

หน้าที่ของผู้เล่นคือทำอย่างไรก็ได้ให้พินทั้งหมดล้มลง และถ้าสามารถล้มพินทั้งหมดลงได้ภายในครั้งแรก (สไตรค์) หรือสามารถล้มพินทั้งหมดได้ภายในครั้งที่สอง (สแปร์) ก็จะได้รับคะแนนพิเศษ ในกรณี่ที่ไม่สามารถล้มพินทั้งหมดได้ภายในครั้งที่สองจะเรียกว่าโอเพ่นเฟรม 

สามารถดูกติการและวิธีการนับคะแนนอย่างละเอียดได้ที่ [wikiHow](https://th.wikihow.com/นับคะแนนโบว์ลิ่ง)

![Bowling Score Table](https://img.pic.in.th/5794a236b7655ba4eef7ad14.png)

ในสไลด์มีอธิบายโปรแกรมเพิ่มเติมดังนี้ ใน 1 เกมจะมี 10 เฟรมและแต่ละเฟรมจะมีจำนวนการโยนตั้งแต่ 1 ถึง 2 ครั้ง ยกเว้นเฟรมที่สิบสามารถมีจำนวนการโยนได้ตั้งแต่ 1, 2 และ 3 ครั้ง ดังนั้นหน้าตาของคลาสและฟังก์ชันตามสไลด์จะได้ออกมาแบบนี้

```go
# game.go

// Game contains a one bowling game data such as frames, score, etc.
type Game struct {
}

// roll records number of knocked down pins.
func (g Game) roll(pins int) {
}

// score calculates and returns a total score of the game.
func (g Game) score() int {
	      return 0
}
```

ฟังก์ชัน `roll` จะถูกเรียกทุกครั้งที่ผู้เล่นโยนบอลตัวแปร `pins` คือจำนวนของพินที่ผู้เล่นสามารถล้มลงได้ และฟังก์ชัน `score` จะถูกเรียกเมื่อจบเกมและตำนวณคะแนนด้วยการลูปทุก ๆ เฟรมเพื่อนับคะแนน

เริ่มต้นด้วยการเขียนเทสกันก่อนตามนี้

```go
# game_test.go

func TestGame(t *testing.T) {
        // Given
        tests := map[string]struct {
                frames   [][]int // contains slice of the knocked down pins number
                expected int
        }{
                "When no roll, should return 0": {
                        expected: 0,
                },
                "When knockout 1 pin per frame, should return 10": {
                        frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
                        expected: 10,
                },
        }

        // When
        for name, test := range tests {
                t.Run(name, func(t *testing.T) {
                        g := Game{}
                        // play the game n frames
                        for _, frame := range test.frames {
                                // roll a ball and knock down the pins
                                for _, pins := range frame {
                                  g.roll(pins)
                                }
                        }

                        score := g.score()
                        if score != test.expected {
                                t.Errorf("expected %d but got: %d", test.expected, score)
                        }
                })
        }
}

```

แน่นอนว่ารันแล้วต้อว _พัง_ อย่างน้อยหนึ่งเคส จากนั้นปรับปรุงโปรแกรมเพื่อให้สามารถคำนวณคะแนนจากจำนวนพินที่ล้มโดยเพิ่มพร็อพเพอร์ตี้ `knockedPins` ที่ `Game`

```go
# game.go

type Game struct {
	      knockedPins int
}
```

จากนั้นแก้ไขฟังก์ชัน `roll` และ `score` ตามนี้


```go
# game.go

func (g *Game) roll(pins int) {
        g.knockedPins += pins
}

func (g Game) score() int {
        return g.knockedPins
}
```

สังเกตที่ฟังก์ชัน `roll` จะเปลี่ยนจาก `func (g Game)` เป็น `func (g *Game)` เพราะต้องบวกจำนวนพินที่ `knockedPins` จาก `pins` ที่ส่งเข้ามา

---

คำนวณคะแนนจากจำนวนพินแล้วถัดมาเป็นคะแนนพิเศษที่ได้จากการเก็บสแปร์ ก่อนอื่นเขียนเทสสำหรับกรณีที่มีการเก็บสแปร์ได้ตามนี้

เมื่อทำสแปร์ได้จะได้รับคะแนนพิเศษของจำนวนพินทีัล้มได้ในครั้งถัดไป ยกตัวอย่างเช่นในรอบที่สองสามารถเก็บสแปร์ได้และการโยนครั้งถัดไป (เฟรมที่สาม) สามารถล้มได้ 1 พิน

คะแนน 1 ตรงนี้จะมาเพิ่มให้ในเฟรมที่สองที่สามารถเก็บสแปร์ได้เป็น 11 คะแนนซึ่งมาจาก 1 + 9 พินที่ล้มได้ + 1 พินที่ล้มได้ในเฟรมที่สาม

```go
# game_test.go

func TestGame(t *testing.T) {
        // Given
        tests := map[string]struct {
                frames   [][]int // contains slice of the knocked down pins number
                expected int
        }{
                ...
                "When able to spare at the 2nd frame, should return 20": {
                        frames:   [][]int{{1, 0}, {1, 9}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
                        expected: 20,
                },
        }

        ...
}
```

ทำการแก้ไขโปรแกรมเพื่อให้สามารถคำนวณคะแนนพิเศษจากการเก็บสแปร์ได้แบบนี้ อันดับแรกสร้าง `Frame` เพื่อใช้เก็บพินที่ล้มได้ในครั้งแรกและครั้งสองของการโยนบอล

```go
# game.go

// Frame contains first and last knocked down pins 
// for using in the total score calculation.
type Frame struct {
	      firstKnockedPins, lastKnockedPins int
}

// isSpared returns true when the first knocked down pins less than 10
// and the sum of first and last knocked down pins equal 10.
func (f Frame) isSpared() bool {
	      return f.firstKnockedPins < 10 && f.firstKnockedPins+f.lastKnockedPins == 10
}

// isStrike returns true when the first knocked down pins is 10.
func (f Frame) isStriked() bool {
	      return f.firstKnockedPins == 10
}
```

จากนั้นแก้ไข `Game` โดยเพิ่ม `times` เพื่อใช้นับจำนวนที่โยนและเปลี่ยน `knockedPins int` เป็น `frames []Frame` แทน

```go
# game.go

type Game struct {
        times  int
        frames []Frame
}
```

จากนั้นแก้ไขฟังก์ชัน `roll` และ `score` ให้คำนวนคะแนนจากจำนวนพินที่ล้มได้ในแต่ละเฟรมแทน

```go
# game.go

func (g *Game) roll(pins int) {
        g.times++
        // this is the first attempt,
        // create a new frame instance and
        // put the number of knocked down pin in it.
        if g.times%2 != 0 {
                g.frames = append(g.frames, Frame{firstKnockedPins: pins})
                return
        }
        // get the latest frame from the list of frames
        // and update its roll.
        g.frames[len(g.frames)-1].lastKnockedPins = pins
}

func (g Game) score() int {
        var totalScore int

        for i, f := range g.frames {
          totalScore += f.firstKnockedPins + f.lastKnockedPins

          // when the current frame able to spare all pins,
          // get the first knocked down pin in the
          // next frame's rolls plus with the total score.
          if f.isSpared() && i+1 < len(g.frames) {
            totalScore += g.frames[i+1].firstKnockedPins
          }
        }

        return totalScore
}
```

---

ถัดมาจะเป็นเรื่องของการคำนวณคะแนนที่ได้จากการทำสไตรค์ วิธีคำนวณจะคล้ายกับตอนทำสแปร์โดยนับคะแนนพิเศษจากการโยนครั้งถัดไปสองครั้ง

ยกตัวอย่างเช่นเฟรมที่สองทำสไตรค์ได้จะได้ 10 คะแนนและในเฟรมที่สามสามารถโยนเก็บพินได้ 1, 2 ครั้งตามลำดับ เฟรมที่สองจะได้คะแนนรวมทั้ง 10 + 1 + 2 = 12 คะแนน

เพิ่มเทสสำหรับเคสสไตรค์ตามนี้

```go
# game_test.go

func TestGame(t *testing.T) {
        // Given
        tests := map[string]struct {
                frames   [][]int // contains slice of the knocked down pins number
                expected int
        }{
                ...
                "When able to strike at the 3rd frame, should return 20": {
                        frames:   [][]int{{1, 0}, {1, 0}, {10}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
                        expected: 20,
                },
                "When able to strike at the 2nd frame and spare at the 3rd frame, should return 39": {
                        frames:   [][]int{{1, 0}, {10}, {1, 9}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
                        expected: 39,
                },
                "When able to strike at the 1st and 2nd frames, should return 46": {
                        frames:   [][]int{{10}, {10}, {1, 3}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
                        expected: 46,
                },
        }

        ...
}
```

จากนั้นมาปรับปรุงโปรแกรมให้รองรับการคำนวณคะแนนสไตรค์ โดยเริ่มจากฟังก์ชัน `roll` ให้ข้ามการโยนครั้งที่สองไปเลยเพราะการทำสไตรค์หมายถึงพินทั้งหมดในเฟรมนั้นล้มหมดแล้ว

```go
# game.go

// roll records number of knocked down pins.
func (g *Game) roll(pins int) {
        ...
        if g.times%2 != 0 {
                // strike!
                // let's skip the second roll of this frame
                if pins == 10 {
                  g.times++
                }

                g.frames = append(g.frames, Frame{firstKnockedPins: pins})
                return
        }
        ...
}
```

เสร็จแล้วแก้ไขฟังก์ชัน `score` เพิ่มเงื่อนไขการตรวจสอบว่าเฟรมนั้นสไตรค์หรือไม่ ถ้าใช่ให้เอาคะแนนจากการโยนสองครั้งถัดไปมารวมเป็นคะแนนพิเศษ

```go
# game.go

// score calculates and returns a total score of the game.
func (g Game) score() int {
        var totalScore int

        for i, f := range g.frames {
                totalScore += f.firstKnockedPins + f.lastKnockedPins

                ...

                // when the current frame able to strike all pins,
                // get the first and second knocked down pins in the
                // next frame's rolls plus with the total score.
                if f.isStriked() && i+1 < len(g.frames) {
                        // in case of the next frame also strike,
                        // then use the next first knocked down pins
                        // of the next 2 frames instead.
                        if g.frames[i+1].isStriked() && i+2 < len(g.frames) {
                                totalScore += g.frames[i+1].firstKnockedPins + g.frames[i+2].firstKnockedPins
                                continue
                        }
                        totalScore += g.frames[i+1].firstKnockedPins + g.frames[i+1].lastKnockedPins
                }
        }

        return totalScore
}
```

ถึงตรงนี้โปรแกรมสามารถคำนวณคะแนนสแปร์และสไตรค์ได้แล้ว 

แต่โบวลิ่งมีคะแนนพิเศษอีกแบบเมื่อทำสแปร์หรือสไตรค์ได้ในเฟรมสุดท้าย เกมจะอนุญาตให้ผู้เล่นโยนบอลอีกหนึ่งหรือสองครั้งถ้าหากทำสแปร์หรือสไตรค์ได้ตามลำดับ

ยกตัวอย่างเช่นถ้าเฟรมสุดท้ายสามารถทำสแปร์ได้ด้วยคะแนน 3 + 7 = 10 และผู้เล่นได้รับอนุญาตให้โยนอีกครั้งและล้มพินได้ 4 เฟรมนี้จะได้คะแนนทั้งหมด 3 + 7 + 4 = 14

หรือถ้าเฟรมสุดท้ายสามารถทำสไตรค์ได้ 10 คะแนนผู้เล่นจะได้รับอนุญาตให้โยนบอลอีกสองครั้งโดยทั้งสองครั้งได้สไตรค์ทั้งคู่แบบนี้คะแนนทั้งหมดจะได้เป็น 10 + 10 + 10 = 30

มาเพิ่มเทสสำหรับเคสสุดท้ายกัน

```go
# game_test.go

func TestGame(t *testing.T) {
        // Given
        tests := map[string]struct {
                frames   [][]int // contains slice of the knocked down pins number
                expected int
        }{
                ...
                "When able to spare at the last frame, should return 24": {
                        frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {3, 7, 5}},
                        expected: 24,
                },
                "When able to strike at the last frame, should return 22": {
                        frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {10, 1, 2}},
                        expected: 22,
                },
                "When able to strike at the last frame and able to keep strike for the next 2 turns, should return 39": {
                        frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {10, 10, 10}},
                        expected: 39,
                },
        }

        ...
}
```

จากนั้นแก้ไข `Frame` เพื่อบันทึกคะแนนพิเศษสำหรับเฟรมสุดท้ายโดยเพิ่ม `lastFrameKnockedPins` แบบนี้

```go
# game.go

type Frame struct {
        firstKnockedPins, lastKnockedPins int
        lastFrameKnockedPins              int
}
```