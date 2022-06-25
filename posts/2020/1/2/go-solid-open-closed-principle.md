---
title: Go SOLID - Open-closed Principle
publish_date: 2020-01-02
---

บล็อกนี้เล่าถึงหลักการ Open-closed หนึ่งในหลักการของ SOLID ที่กล่าวไว้โดย Bertrand Meyer ในหนังสือเรื่อง [Object Oriented Sofrware Construction](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle#cite_note-1) แปลได้ใจความว่า "เอนทิตี้ใด ๆ ในซอฟต์แวร์เช่นคลาส, โมดูล, ฟังก์ชัน ควรเปิดเพื่อให้ต่อยอดได้ (extension) แต่ปิดเพื่อป้องกันไม่ให้แก้ไข (modification)"

> Software entities (classes, modules, functions, etc.) should be open for extension, but closed for modification

Open-closed เป็นหลักการที่พบเห็นได้ค่อนข้างบ่อยหรือบางทีก็ใช้งานอยู่โดยไม่รู้ตัวด้วยซ้ำ ลองมาดูตัวอย่างของหลักการนี้ในแบบฉบับของ Go กันดีกว่า

## สารบัญ

- [Go SOLID - Single Responsibility Principle](/2020/1/10/go-solid-single-responsibility-principle)
- Go SOLID - Open-closed Principle
- [Go SOLID - Interface Segregation Principle](/2019/12/20/go-solid-interface-segregation-principle)

---

ยกตัวอย่างฟังก์ชันสำหรับแสดงผลพื้นที่ของรูปทรงชื่อว่า `printArea` ในตอนแรกสุดฟังก์ชันรู้จักแค่รูปสี่เหลี่ยมจตุรัสเพียงอย่างเดียว ซึ่งอิมพลิเมนต์การคำนวณพื้นที่ด้วยสูตรที่เรารู้จักกันดีคือด้านกว้างคูณกับด้านยาว

```go
import "log"

type square struct { width float64 }

// printArea displays a shape area on the stand output.
func printArea(o square) {
        log.Printf("shape area is: %.2f\n", o.width*o.width)
}
```

ต่อมาถ้าเกิดว่าต้องการให้ฟังก์ชัน `printArea` สามารถใช้งานกับสี่เหลี่ยมผืนผ้าได้ด้วย ก็อาจจะแก้ฟังก์ชันให้รับเป็นแบบนี้แทน

```go
type rectangle struct { width, height float64 }

func printArea(o any) {
        var area float64
        switch s := o.(type) {
        case square: 
                area = s.width*s.width
        case rectangle: 
                area = s.width*s.height
        }
        log.Printf("shape area is: %.2f\n", area)
}
```

จะเห็นว่าจากตัวอย่างแค่เพิ่ม `rectangle` เข้ามาก็ต้องมาแก้คอยแก้ไขฟังก์ชัน `printArea` อยู่ตลอด แบบนี้คงไม่ดีแน่ ลองเอาหลักการ open-closed มาปรับใช้ดังนี้

ก่อนอื่นทำให้ฟังก์ชัน `printArea` เปิดเพื่อรับต่อยอด (open for extension) ด้วยการรับอินเตอร์เฟส `shape` แทน

```go
type shape interface { 
        // calculate and return a shape area for each type of the shape
        area() float64 
}

func printArea(s shape) { ... }
```

ซึ่งอินเตอร์เฟส `shape` จะบังคับให้แต่ละรูปทรงต้องอิมพลิเมนต์ฟังก์ชัน `area` เพื่อบอกว่ารูปทรงนั้น ๆ จะมีขนาดพื้นที่เท่าไร เป็นการปิดไม่ให้แก้ไข (closed for modification) สูตรการคำนวณพื้นที่ของแต่ละรูปทรงที่ฟังก์ชัน `printArea` อีกด้วย

```go
func (s square) area() float64 {
        return s.width * s.width
}

func (s rectangle) area() float64 {
        return s.width * s.height
}
```

---

หลักการ open-closed เดิมที่ถูกออกแบบเพื่อใช้งานระหว่างคลาสและซูเปอร์คลาสในภาษาตระกูล OOP เป็นหลัก แต่ภายหลังได้เปลี่ยนมาเป็นการใช้งานอินเตอร์เฟสแทนเพื่อความยืดหยุ่นที่มากขึ้น