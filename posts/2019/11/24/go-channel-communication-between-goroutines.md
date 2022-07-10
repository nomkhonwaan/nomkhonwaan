---
title: Channel กับการสื่อสารระหว่าง Goroutines
publish_date: 2019-11-24
---

Go มี built-in concurrency ที่เรียกว่า Goroutines การทำงานคล้ายกับ Thread ที่อยู่ใน Java แต่มีขนาดเล็กและเบากว่า ซึ่งตัว Go เองเคลมว่า [สามารถรันได้ถึงหนึ่งแสน Goroutines](https://go.dev/doc/faq#goroutines) Go อนุญาตให้ Goroutines สามารถสื่อสารระหว่างกันได้ผ่านทางตัวแปรประเภทแชนแนล มาลองดูว่าแชนแนลใน Go มีกี่ประเภทและสามารถใช้งานอะไรได้บ้าง

> It is practical to create hundreds of thousands of goroutines in the same address space.

---

1. วิธีสร้างตัวแปรแชนแนล

แชนแนลสามารถสร้างจากตัวแปรประเภทไหนก็ได้โดยใส่คีย์เวิร์ด `chan` ข้างหน้าและใช้คำสั่ง `make` แบบนี้

```go
bufferedCh := make(chan string, 10)
unbufferedCh := make(chan struct{})
```

ตัวแปรแชนแนลสามารถสร้างได้สองรูปแบบคือมีบัฟเฟอร์และไม่มีบัฟเฟอร์ ตัวอย่างเช่นตัวแปร `bufferedCh` ตอนสร้างระบุประเภทเป็น `string` มีขนาดบัฟเฟอร์​ 10 ในขณะที่ `unbufferedCh` เป็นประเภท `struct{}` และไม่ระบุบัฟเฟอร์

---


2. ความแตกต่างระหว่างแชนแนลที่มีกับไม่มีบัฟเฟอร์?

แชนแนลที่มีบัฟเฟอร์ให้นึกภาพร้านซูชิจานเวียนที่เชฟสามารถปั้นซูชิทีละหลาย ๆ คำและใส่ลงมาบนสายพานโดยที่จำนวนของซูชิที่อยู่บนสายพานได้ขึ้นอยู่กับความยาวของสายพาน ตราบเท่าที่สายพานยังมีที่ว่างเซฟก็สามารถปั้นซูชิใส่ลงมาได้เรื่อย ๆ เหมือนกันกับแชนแนลที่มีบัฟเฟอร์ถ้ายังมีที่ว่างเราก็ยังสามารถใส่ค่าเข้าไปในแชนแนลได้เรื่อย ๆ โดยไม่ต้องมีคนรอรับ

แชนแนลที่ไม่มีบัฟเฟอร์ให้นึกภาพร้านอาหารญี่ปุ่นโอมากาเสะ (Omakase) ตามปกติของการทานร้านโอมากาเสะเซฟจะปั้นซูชิคำถัดไปเมื่อลูกค้าทานคำที่เสิร์ฟในจานหมดก่อนเพื่อรักษาความสดอร่อยของซูชิ เหมือนกันกับแชนแนลที่ไม่มีบัฟเฟอร์เราจะไม่สามารถส่งข้อมูลถัดไปเข้าตัวแปรแชนแนลได้ถ้าข้อมูลก่อนหน้าไม่ถูกดึงออกไปก่อน

ตัวอย่างโค้ดด้านล่างนี้แสดงให้เห็นดึงการเกิด deadlock เพราะไม่มีการดึงข้อมูลออกจากตัวแปร `unbufferedCh` ก่อนส่งค่าเข้าไป

```go
package main

func main() {
        unbufferedCh := make(chan struct{})

        // deadlock!
        unbufferedCh <- struct{}{}
}
```

[Go Playground](https://go.dev/play/p/XkknCPbPmVB)

แผนภาพด้านล่างนี้แสดงถึงการรับส่งข้อมูลระหว่าง Goroutines ผ่านตัวแปรแชนแนลที่ไม่มีบัฟเฟอร์ สังเกตว่าถ้าเป็นการรับ-ส่งข้อมูลผ่านแชนแนลที่ไม่มีบัฟเฟอร์แต่อยู่ต่าง Goroutines กันจะไม่เกิด deadlock

![Unbuffered channel](https://img.pic.in.th/channel-and-goroutines-unbuffered-channel-5df19d68887f91e187f9b02e5d1ed5a18d9896c0.png)

ลองแก้ไขโค้ดตัวอย่างข้างบนให้สามารถรับ-ส่งข้อมูลได้โดยไม่เกิด deadlock ได้แบบนี้

```go
package main

func main() {
        unbufferedCh := make(chan struct{})

        go func(receiveOnlyCh <-chan struct{}) {
                <-receiveOnlyCh
        }(unbufferedCh)

        unbufferedCh <- struct{}{}
}
```

ในขณะที่ตัวแปรแชนแนลที่มีบัฟเฟอร์สามารถนั้นสามารถส่งข้อมูลเข้าไปได้ก่อนโดยไม่ต้องรอให้มีการรับใน Goroutines เดียวกันได้เลยแบบนี้

```go
package main

func main() {
        bufferedCh := make(chan struct{},  10)
        bufferedCh <- struct{}{}
        bufferedCh <- struct{}{}

        <-bufferedCh
        <-bufferedCh
}
```

แต่ถ้าเกิดว่ามีการรอรับมากกว่าที่ส่งเข้าไปใน Goroutines เดียวกันก็ยังเกิด deadlock ได้

```go
package main

func main() {
        bufferedCh := make(chan struct{}, 10)
        bufferedCh <- struct{}{}
        bufferedCh <- struct{}{}

        <-bufferedCh
        <-bufferedCh
        <-bufferedCh
}
```

[Go Playground](https://go.dev/play/p/DeF_5nyXYHA)

แผนภาพแสดงตัวอย่างการรับ-ส่งข้อมูลผ่านแชนแนลที่มีบัฟเฟอร์

![Buffered channel](https://img.pic.in.th/channel-and-goroutines-buffered-channel-5df19d9d887f91e187f9b02f.png)

---

3. ทิศทางของแชนแนล

ตามปกติแล้วตัวแปลแชนแนลสามารถรับ-ส่งข้อมูลได้ทั้งสองทาง แต่ถ้าต้องการให้รับหรือส่งได้ทางเดียวสามารถประกาศทิศทางของแชนแนลได้โดยการระบุด้วยเครื่องหมาย `<-` ที่ด้านหน้าหรือหลังคำว่า `chan` แบบนี้

```go
package main

func main() {
        unbufferdCh := make(chan struct{})
        receiveOnly(unbufferdCh)
        sendOnly(unbufferdCh)
}

// receiveOnly accepts read-only channel.
func receiveOnly(receiveOnlyCh <-chan struct{}) {}

// sendOnly accepts write-only channel.
func sendOnly(sendOnlyCh chan<- struct{}) {}
```

ถ้าพยายามจะส่งส่งข้อมูลเข้าแชนแนลที่รับทางเดียว Go จะตรวจเจอตั้งแต่ตอนคอมไพล์และฟ้องข้อผิดพลาดแบบนี้

```go
// receiveOnly accepts read-only channel.
func receiveOnly(receiveOnlyCh <-chan struct{}) {
        // try to send data to read-only channel
        receiveOnlyCh <- struct{}{}
}
```
```
./prog.go:12:2: invalid operation: cannot send to receive-only channel receiveOnlyCh (variable of type <-chan struct{})

Go build failed.
```

---

4. Fan-out, fan-in

หนึ่งในรูปแบบการใช้งานแชนแนลโดยส่งตัวแปรแชนแนลผ่านแต่ละฟังก์ชันเพื่อทำงานกับข้อมูลที่อยู่ข้างในแชนแนล 

โดยต้นทางฟังก์ชันที่สร้างแชนแนลและส่งออกเรียกว่า _fan-out_ ส่วนอีกทางฟังก์ชันที่รับแชนแนลเข้ามาและรวมผลลัพธ์ให้เหลือตัวเดียวเรียกว่า _fan-in_ 

![Fan-out, fan-in](https://img.pic.in.th/channel-and-goroutines-fan-out-fan-in-5df19dd4887f91e187f9b030.png)

ลองดูตัวอย่างโค้ดคำนวณพื้นที่สี่เหลี่ยมด้านล่างนี้

---
#go #goroutines