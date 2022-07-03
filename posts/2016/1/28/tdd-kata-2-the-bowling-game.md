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
                        frames:   [][]int{{1}, {1}, {1}, {1}, {1}, {1}, {1}, {1}, {1}, {1}},
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

แน่นอนว่ารันแล้วต้อว _พัง_ อย่างน้อยหนึ่งเคส