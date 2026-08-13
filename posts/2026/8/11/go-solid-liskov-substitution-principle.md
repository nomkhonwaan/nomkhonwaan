---
title: Go SOLID - Liskov Substitution Principle
publish_date: 2026-08-11
tags: ['go', 'solid']
---

บล็อกนี้เล่าถึงหลักการ Liskov Substitution Principle (LSP) หนึ่งในหลักการของ SOLID ที่ถูกเสนอครั้งแรกโดย Barbara Liskov ในปี 1987 และถูกนำมาขยายความต่อในบริบทของ [SOLID](https://blog.cleancoder.com/uncle-bob/2020/10/18/Solid-Relevance.html) โดย Robert C. Martin หรือ Uncle Bob สรุปใจความสั้น ๆ ได้ว่า "คลาสลูกจะต้องสามารถถูกนำมาใช้แทนคลาสแม่ได้โดยที่โปรแกรมยังคงทำงานได้อย่างถูกต้องสมบูรณ์"

> Functions that use pointers or references to base classes must be able to use objects of derived classes without knowing it.

หรือนิยามดั้งเดิมของ [Barbara Liskov](https://en.wikipedia.org/wiki/Barbara_Liskov) เอง:

> Let $q(x)$ be a property provable about objects $x$ of type $T$. Then $q(y)$ should be true for objects $y$ of type $S$ where $S$ is a subtype of $T$.

ฟังดูเป็นทางการและแอบเข้าใจยากนิดหน่อยใช่ไหม? ถ้าแปลแบบง่าย ๆ คือ "เมื่อเราเขียนโปรแกรมโดยอ้างอิงเบสไทป์ (Base Type) เราควรส่งซับไทป์ (Subtype) เข้าไปแทนได้เลย โดยที่โปรแกรมไม่ต้องเพิ่มเงื่อนไขพิเศษหรือเกิดพฤติกรรมแปลก ๆ" เพื่อให้เห็นภาพชัดขึ้น ลองมาดูตัวอย่างพร้อมโค้ดกันดีกว่า

## สารบัญ

- [Go SOLID - Single Responsibility Principle](/2020/1/10/go-solid-single-responsibility-principle)
- [Go SOLID - Open-closed Principle](/2020/1/2/go-solid-open-closed-principle)
- [Go SOLID - Liskov Substitution Principle](/2026/8/11/go-solid-liskov-substitution-principle)
- [Go SOLID - Interface Segregation Principle](/2022/5/7/go-solid-interface-segregation-principle)
- [Go SOLID - Dependency Inversion Principle](/2026/8/14/go-solid-dependency-inversion-principle)

---

เพื่อให้เห็นภาพของ LSP ชัดขึ้น ลองมาดูตัวอย่างใกล้ตัวอย่างระบบแตะบัตรเข้า MRT กัน

สมมติว่าเรามีระบบประตูทางเข้าที่ใช้แนวคิดร่วมกันทั้ง MRT และ BTS โดยเริ่มจากออกแบบอินเตอร์เฟสกลางชื่อ `TransitCard` แบบนี้

```go
type TransitCard interface {
	// ChargeFare should deduct exactly fare from the card.
	ChargeFare(fare float64) error
}

func EnterMRTGate(card TransitCard, fare float64) error {
	return card.ChargeFare(fare)
}
```

หน้าตาแบบนี้ดูไม่มีปัญหาอะไร และถ้าเป็นบัตร MRT แบบเติมเงินก็อาจอิมพลิเมนต์ประมาณนี้

```go
type MRTCard struct {
	balance float64
}

func (c *MRTCard) ChargeFare(fare float64) error {
	if fare <= 0 {
		return errors.New("invalid fare")
	}

	if c.balance < fare {
		return errors.New("insufficient balance")
	}

	c.balance -= fare
	return nil
}
```

และถ้าต้องรองรับการแตะเข้าแบบ EMV (บัตรเครดิต/เดบิต) สำหรับ MRT ด้วย ก็ยังดูไม่มีอะไรน่าห่วง

```go
type EMVCard struct {
	balance float64
}

func (c *EMVCard) ChargeFare(fare float64) error {
	if fare <= 0 {
		return errors.New("invalid fare")
	}

	if c.balance < fare {
		return errors.New("insufficient balance")
	}

	c.balance -= fare
	return nil
}
```

ปัญหาเริ่มเกิดตอนทีมต้องเพิ่มฟีเจอร์เข้า BTS ซึ่ง BTS ต้องเช็คแฟลก (flag) เพิ่มเติมก่อนอนุญาตให้เข้า ทีมพัฒนาเลยอยากนำฟังก์ชันเดิมที่รับ `TransitCard` กลับมาใช้ และเขียนออกมาประมาณนี้

```go
func EnterBTSGate(card TransitCard, fare float64) error {
	fc, ok := card.(interface{ HasBTSFlag() bool })
	if !ok || !fc.HasBTSFlag() {
		return errors.New("card is not allowed on BTS")
	}

	return card.ChargeFare(fare)
}
```

ถ้าลองดูฝั่งเรียกใช้งาน จะเห็นภาพชัดขึ้น

```go
func main() {
	card := &MRTCard{
		balance: 500,
	}

	if err := EnterBTSGate(card, 20); err != nil {
		log.Println("enter BTS failed:", err)
	}
}
```

จากโค้ดนี้จะเห็นว่า ถึงแม้ `MRTCard` กับ `EMVCard` จะใช้ได้ปกติกับ MRT แต่พอเข้าเส้นทางของ BTS ฟังก์ชันที่รับ `TransitCard` กลับต้องเพิ่มเงื่อนไขพิเศษ และบางบัตรก็ถูกปฏิเสธทันที ทั้งที่ลายเซ็นของเมธอดเหมือนกัน นี่เป็นสัญญาณว่าการออกแบบนามธรรมเดิมกว้างเกินไป จนผู้ใช้เบสไทป์ต้องรู้รายละเอียดของซับไทป์ ซึ่งขัดกับเจตนาของ LSP

---

วิธีแก้คือแยกสัญญาให้ชัดตั้งแต่แรกว่า บัตรที่เข้า BTS ได้ต้องมีความสามารถเช็คแฟลกเพิ่มเติม ไม่ใช่รวมทุกอย่างไว้ใน `TransitCard` ตัวเดียว

```go
type TransitCard interface {
	// ChargeFare should deduct exactly fare from the card.
	ChargeFare(fare float64) error
}

type FlagCheckedTransitCard interface {
	TransitCard
	HasBTSFlag() bool
}

func EnterMRTGate(card TransitCard, fare float64) error {
	return card.ChargeFare(fare)
}

func EnterBTSGate(card FlagCheckedTransitCard, fare float64) error {
	if !card.HasBTSFlag() {
		return errors.New("card is not allowed on BTS")
	}

	return card.ChargeFare(fare)
}

type EMVCard struct {
	balance float64
	btsFlag bool
}

func (c *EMVCard) HasBTSFlag() bool {
	return c.btsFlag
}

func (c *EMVCard) ChargeFare(fare float64) error {
	if fare <= 0 {
		return errors.New("invalid fare")
	}

	if c.balance < fare {
		return errors.New("insufficient balance")
	}

	c.balance -= fare
	return nil
}
```

เมื่อแก้แบบนี้ ฟังก์ชันแต่ละตัวจะพึ่งพาเฉพาะสิ่งที่จำเป็นจริง ๆ

- `EnterMRTGate` รับแค่ `TransitCard`
- `EnterBTSGate` รับ `FlagCheckedTransitCard`

ผลคือผู้ใช้แต่ละฟังก์ชันไม่ต้องเดาแล้วว่าซับไทป์ตัวไหนมีพฤติกรรมพิเศษแอบอยู่ และไม่ต้องเขียน type assertion แทรกกลางทางตลอดเวลา

อย่างไรก็ตาม LSP ไม่ได้หมายถึงแค่ลายเซ็นเมธอดที่ตรงกันเท่านั้น แต่ยังรวมถึงเงื่อนไขก่อนใช้งานและผลลัพธ์หลังใช้งานด้วย เช่น

- subtype ไม่ควรต้องการเงื่อนไขก่อนใช้งานที่เข้มงวดกว่าเบสไทป์
- subtype ไม่ควรคืนค่าผลลัพธ์ที่ทำให้สัญญาเดิมของระบบอ่อนลง
- subtype ไม่ควรมีผลข้างเคียงแปลก ๆ ที่ผู้ใช้เบสไทป์ไม่ได้คาดคิด

ยกตัวอย่างเช่น ถ้าสัญญาของ `ChargeFare` คือ “ตัดค่าโดยสารตามจำนวนที่ระบุหรือล้มเหลว” แต่มี implementation ไหนสักตัวแอบต้องการเงื่อนไขพิเศษที่อินเตอร์เฟสไม่ได้บอกไว้ แบบนั้นต่อให้โค้ดคอมไพล์ผ่าน ก็ถือว่าน่าสงสัยว่าอาจละเมิด LSP

---

ในโลกของ Go เราไม่ได้มี inheritance แบบคลาสเหมือนภาษา OOP ดั้งเดิม แต่เรายังเจอปัญหาแบบ LSP ได้เสมอผ่านการใช้อินเตอร์เฟส เพราะอินเตอร์เฟสไม่ได้บอกแค่ว่า “ต้องมีเมธอดอะไรบ้าง” มันยังมีสัญญาเรื่องพฤติกรรมแนบมาด้วย

ดังนั้นเวลาออกแบบอินเตอร์เฟสหรืออ้างอิงสิ่งอื่นเข้ามาใช้งาน อย่าหยุดแค่ถามว่า “type นี้มีเมธอดครบไหม” แต่ควรถามต่อด้วยว่า “มันมีพฤติกรรมเหมือนที่ผู้ใช้ type นี้คาดหวังจริงหรือเปล่า” ถ้าคำตอบคือไม่ นั่นอาจเป็นสัญญาณว่าการออกแบบเริ่มหลุดจากหลัก LSP แล้ว
