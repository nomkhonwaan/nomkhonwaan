---
title: Go SOLID - Interface Segregation Principle
publish_date: 2022-05-07
---

บล็อกนี้เล่าถึงหลักการ Interface Segreagation Principle (ISP) หนึ่งในหลักการของ SOLID ที่กล่าวไว้โดย Robert C. Martin หรือ Uncle Bob ในบทความที่ตอบ[จดหมาย](https://blog.cleancoder.com/uncle-bob/2020/10/18/Solid-Relevance.html)เอาไว้ดังนี้

> Keep interfaces small so that users don’t end up depending on things they don’t need.

หมายความว่าพยายามทำให้อินเตอร์เฟสมีขนาดเล็กท่าที่จำเป็นต่อการใช้งานเพื่อผู้ใช้จะได้ไม่ต้องสนใจสิ่งอื่น ๆ ลองมาดูตัวอย่างในแบบของ Go กัน

## สารบัญ

- [Go SOLID - Single Responsibility Principle](/2020/1/10/go-solid-single-responsibility-principle)
- [Go SOLID - Open-closed Principle](/2020/1/2/go-solid-open-closed-principle)
- [Go SOLID - Interface Segregation Principle](/2022/5/7/go-solid-interface-segregation-principle)

---

ยกตัวอย่างระบบหนึ่งต้องมีการเก็บล็อกของการทำรายการลงไฟล์ อาจจะประกาศฟังก์ชัน `writeLog` หน้าตาแบบนี้

```go
// writeLog writes a given arguments data into the file.
// An error will be returned when log writing failed.
func writeLog(f *os.File, args ...any) error {
        ...
}
```

ฟังก์ชันทำงานได้ดีแต่ปัญหามันอยู่ที่การเขียนยูนิตเทส ซึ่งเป็นไปไม่ได้เลยที่จะไม่ต้องเปิดไฟล์จริง ๆ ขึ้นมาเขียนตอนเทส มาลองแก้ปัญหาด้วยการเปลี่ยน `*os.File` เป็นอินเตอร์เฟส `io.WriteCloser` แบบนี้

```go
func writeLog(f io.WriteCloser, args ...any) error {
        ...
}
```

OK ตอนนี้สามารถเขียนยูนิตเทสกับฟังก์ชัน `writeLog` ได้แล้ว.. แต่ถ้าพิจารณาจากหลักการ ISP การใช้อินเตอร์เฟส `io.WriteCloser` ดูว่าจะเกินความจำเป็นของฟังก์ชัน `writeLog` ไปสักหน่อย

หน้าที่หลักของฟังก์ชัน `writeLog` คือทำหน้าที่แค่เขียนล็อกไม่จำเป็นต้องทำหน้าที่ปิดไฟล์ ซึ่งว่ากันตามตรงตัวฟังก์ชันไม่จำเป็นต้องรู้ด้วยซ้ำว่าตัวแปร `f` นั้นจริง ๆ แล้วเป็นไฟล์พอยน์เตอร์หรือไม่ ดังนั้นถ้าเปลี่ยนจาก `io.WriteCloser` เป็น `io.Writer` น่าจะตรงกับสิ่งที่ฟังก์ชัน `writeLog` ต้องการมากกว่า

```go
func writeLog(f io.Writer, args ...any) error {
        ...
}
```

ถ้าเขียนโค้ดเต็ม ๆ ในส่วนของการเรียกใช้งานฟังก์ชัน `writeLog` จะได้ออกมาประมาณนี้

```go
package main

import (
        "fmt"
        "io"
        "log"
        "os"
        "path/filepath"
)

func main() {
        f, err := os.OpenFile(filepath.Join(os.TempDir(), "application.log"), os.O_CREATE|os.O_WRONLY, 0644)
        if err != nil {
                log.Fatal(err)
        }
        defer f.Close()

        err = writeLog(f, "hello, world!")
        if err != nil {
                log.Fatal(err)
        }
}

func writeLog(f io.Writer, args ...any) error {
        _, err := fmt.Fprint(f, args...)
        return err
}
```

---
#go #solid