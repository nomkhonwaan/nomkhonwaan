---
title: Channel กับการสื่อสารระหว่าง Goroutines
publish_date: 2019-11-24
tags: ['go', 'goroutines']
---

Go มี built-in concurrency ที่เรียกว่า Goroutines การทำงานคล้ายกับ Thread ที่อยู่ใน Java แต่มีขนาดเล็กและเบากว่า ซึ่ง Go เคลมว่า[สามารถรันได้ถึงหนึ่งแสน Goroutines](https://go.dev/doc/faq#goroutines) 

> It is practical to create hundreds of thousands of goroutines in the same address space.

แต่สิ่งที่จะเล่าถึงในบล็อกนี้คือ Goroutines, สิ่งที่ใช้ในการสื่อสารกันระหว่าง Goroutines เรียกว่าตัวแปรแชนแนล มาดูว่าแชนแนลใน Go สามารถใช้อะไรได้บ้าง

## ตัวแปรแชนแนล

แชนแนลสามารถสร้างจากตัวแปรประเภทไหนก็ได้โดยใส่คีย์เวิร์ด `chan` ข้างหน้าและใช้คำสั่ง `make` แบบนี้

```go
bufferedCh := make(chan string, 10)
unbufferedCh := make(chan struct{})
```

ตัวแปรแชนแนลสามารถสร้างได้สองรูปแบบคือมีบัฟเฟอร์และไม่มีบัฟเฟอร์ ตัวอย่างเช่นตัวแปร `bufferedCh` ตอนสร้างระบุประเภทเป็น `string` มีขนาดบัฟเฟอร์​ 10 ในขณะที่ `unbufferedCh` เป็นประเภท `struct{}` และไม่ระบุบัฟเฟอร์

_ความแตกต่างระหว่างแชนแนลที่มีกับไม่มีบัฟเฟอร์?_

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

![Unbuffered channel](https://img.pic.in.th/Channel-and-Goroutines-Unbuffered-Channel.drawio.png)

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

![Buffered channel](https://img.pic.in.th/Channel-and-Goroutines-Buffered-Channel.drawio.png)

---

_ทิศทางของแชนแนล_

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

## Fan-out, fan-in

หนึ่งในรูปแบบการใช้งานแชนแนลโดยส่งตัวแปรแชนแนลผ่านแต่ละฟังก์ชันเพื่อทำงานกับข้อมูลที่อยู่ข้างในแชนแนล 

โดยต้นทางฟังก์ชันที่สร้างแชนแนลและส่งออกเรียกว่า _fan-out_ ส่วนอีกทางฟังก์ชันที่รับแชนแนลเข้ามาและรวมผลลัพธ์ให้เหลือตัวเดียวเรียกว่า _fan-in_ 

ลองดูตัวอย่างโค้ดด้านล่างนี้

```go
package main

import (
        "log"
        "sync"
)

func main() {
        input := generate(2, 3, 4)
        output1 := square(input)
        output2 := square(input)

        final := merge(output1, output2)

        for n := range final {
                log.Println(n)
        }
}

func generate(numbers ...int) <-chan int {
        out := make(chan int)
        go func() {
                for _, n := range numbers {
                        out <- n
                }
                close(out)
        }()
        return out
}

func square(input <-chan int) <-chan int {
        out := make(chan int)
        go func() {
                for n := range input {
                        out <- n * n
                }
                close(out)
        }()
        return out
}

func merge(outputs ...<-chan int) <-chan int {
        out := make(chan int)
        wg := sync.WaitGroup{}
        wg.Add(len(outputs))

        go func() {
                wg.Wait()
                close(out)
        }()

        for _, output := range outputs {

                go func(output <-chan int) {
                        for n := range output {
                                out <- n
                        }
                        wg.Done()
                }(output)
        }

        return out
}
```

[Go Playground](https://go.dev/play/p/NibvAdKcpXG)

ฟังก์ชัน `generate` ทำหน้านี้รับ `[]int` เข้ามาแล้วส่งต่อให้กับตัวแปรแชนแนล `out` จากนั้นฟังก์ชัน `square` รับตัวแปรแชนแนล `out` จาก `generate` แล้วนำค่าที่ได้มายกกำลังสองและรีเทิร์นกลับเป็นแชนแนลเหมือนกัน ณ ตรงนี้ฟังก์ชัน `square` จะถูกเรียกสองครั้งโดยแต่ละตัวจะแทนหนึ่ง Goroutines ซึ่งหมายความว่าในการรันแต่ละครั้งจำนวนอินพุตที่ `square` ได้รับอาจจะแตกต่างกันไป

สุดท้ายคือฟังก์ชัน `merge` ที่ทำหน้าที่เอาผลลัพธ์ที่ได้จาก `square` มารวมกันให้ออกเพียงหนึ่งแชนแนลเท่านั้นและส่งไป `log.Println` ที่ `main` ถ้าลองรันโค้ดดูจะพบว่าลำดับของตัวเลขที่แสดงผลออกมานั้นไม่ได้เรียงกันตามอินพุตที่ใส่เข้าไป สาเหตุมาจากว่าตอนที่ส่งไปยัง `square` นั้นตัวเลขได้ถูกสุ่มส่งให้กับ `output1` หรือ `output2` และทำงานจบไม่พร้อมกันทำให้ผลลัพธ์สุดท้ายอาจจะไม่ได้เรียงกันตามอินพุตที่ใส่ไป

ถ้าจะให้เห็นภาพมากขึ้นลองแก้ไขโปรแกรมให้รับอินพุตเป็นตัวเลขจำนวนหลาย ๆ ตัวแล้วสังเกตผลลัพธ์ที่ได้จากการรันในแต่ละครั้งดู

แผนภาพด้านล่างแสดงให้เห็นว่าการส่งข้อมูลไปยัง `square` อินพุตสามารถถูกแบ่งออกเป็นสองกลุ่มคือ `2, 3` และ `4` โดยกลุ่มแรกจะได้ออกมาเป็น `output1` และกลุ่มสองจะได้ออกมาเป็น `output2`

![Fan-out, fan-in](https://img.pic.in.th/Channel-and-Goroutines-Fan-out-fan-in.drawio.png)

## Futures

ฟิวเจอร์เป็นรูปแบบของการใช้งาน Goroutines ที่คาดหวังว่าจะได้ผลลัพธ์กลับมาในอนาคตซึ่งในระหว่างที่รอผลลัพธ์สามารถทำงานอย่างอื่นรอได้ รูปแบบของฟิวเจอร์จะพบได้ใน Scala หรือ Promise ใน JavaScript

ตัวอย่างโค้ดด้านล่างนี้มีการเรียกไปยังเว็บไซต์​ JSONPlaceholder เพื่อเอาข้อมูลโพสต์กลับมา​ โดยฟังก์ชัน `request` จะยังไม่ได้รีเทิร์น `data` กลับมาทันทีแต่เป็น `chan data` แทน

```go
package main

import (
        "encoding/json"
        "fmt"
        "io"
        "net/http"
)

type data struct {
        body struct {
                UserID int    `json:"userId"`
                ID     int    `json:"id"`
                Title  string `json:"title"`
                Body   string `json:"body"`
        }
        err error
}

func main() {
        future, err := request(http.MethodGet, "https://jsonplaceholder.typicode.com/posts/1", nil)
        if err != nil {
                panic(err)
        }

        // Do other things

        d := <-future
        if d.err != nil {
                panic(d.err)
        }

        fmt.Printf("%#v\n", d.body)
}

func request(method string, url string, body io.Reader) (<-chan data, error) {
        r, err := http.NewRequest(method, url, body)
        if err != nil {
                return nil, err
        }

        future := make(chan data)

        go func() {
                rs, err := http.DefaultClient.Do(r)
                if err != nil {
                        future <- data{err: err}
                        return
                }
                defer rs.Body.Close()

                var d data
                d.err = json.NewDecoder(rs.Body).Decode(&d.body)

                future <- d
                close(future)
        }()

        return future, nil
}
```

[Go Playground](https://go.dev/play/p/oPrSZ9HY3Uf)

แผนภาพด้านล่างแสดงให้เห็นถึงฟังก์ชัน `request` ที่รีเทิร์นแชนแนลกลับมาให้ `main` ก่อนเพื่อให้ `main` สามารถทำงานอย่างอื่นระหว่างรอผลลัพธ์จาก JSONPlaceholder 

![Futures](https://img.pic.in.th/Channel-and-Goroutines-Futures.drawio.png)

---

อ้างอิง

1. [The Go Blog - Go Concurrency Patterns: Pipelines and cancellation](https://go.dev/blog/pipelines)
2. [Thejas Babu - Concurrency Patterns: Golang](https://medium.com/@thejasbabu/concurrency-patterns-golang-5c5e1bcd0833)