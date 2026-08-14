---
title: Go SOLID - Dependency Inversion Principle
publish_date: 2026-08-14
tags: ['go', 'solid']
---

บล็อกนี้เล่าถึงหลักการ Dependency Inversion (DIP) ซึ่งเป็นหลักการสุดท้ายของ SOLID ที่กล่าวไว้โดย Robert C. Martin หรือ Uncle Bob เช่นเคย ฟังจากชื่อก็อาจจะเดาไม่ออกว่ามันคืออะไรกันแน่ มาลองดูคำนิยามกันก่อนดีกว่า

> A. High-level modules should not import anything from low-level modules. Both should depend on abstractions (e.g., interfaces).

> B. Abstractions should not depend on details. Details (concrete implementations) should depend on abstractions.

แปลง่าย ๆ ได้ว่า "โมดูลระดับสูงไม่ควรขึ้นตรงกับโมดูลระดับต่ำ แต่ทั้งคู่ควรขึ้นกับ abstraction และ abstraction ไม่ควรขึ้นตรงกับรายละเอียด แต่รายละเอียดควรขึ้นกับ abstraction"

อ่านแล้วอาจจะงง ๆ ว่ามันคืออะไรกันแน่? ขออธิบายเพิ่มเติมอีกนิดว่าหลักการนี้แตกต่างจาก Inversion of Control (IoC) หรือ Dependency Injection (DI) ที่เราเคยได้ยินกันบ่อย ๆ จริง ๆ แล้วมันคนละเรื่องเดียวกัน DIP คือหลักการ (principle) ส่วน DI และ IoC คือวิธีการนำหลักการไปใช้งานนั่นเอง

## สารบัญ

- [Go SOLID - Single Responsibility Principle](/2020/1/10/go-solid-single-responsibility-principle)
- [Go SOLID - Open-closed Principle](/2020/1/2/go-solid-open-closed-principle)
- [Go SOLID - Liskov Substitution Principle](/2026/8/11/go-solid-liskov-substitution-principle)
- [Go SOLID - Interface Segregation Principle](/2022/5/7/go-solid-interface-segregation-principle)
- [Go SOLID - Dependency Inversion Principle](/2026/8/14/go-solid-dependency-inversion-principle)

---

ยกตัวอย่างการเขียนระบบแจ้งเตือนอย่างง่าย `NotificationService` ที่ต้องส่งอีเมลไปยังผู้ใช้เมื่อเกิดเหตุการณ์บางอย่างขึ้น

```go
package notification

import "log"

type EmailSender struct{}

func (e *EmailSender) SendEmail(to, subject, body string) {
        log.Printf("sending email to %s: %s", to, subject)
}

type NotificationService struct {
        sender *EmailSender
}

func NewNotificationService() *NotificationService {
        return &NotificationService{sender: &EmailSender{}}
}

func (n *NotificationService) Notify(to, message string) {
        n.sender.SendEmail(to, "Notification", message)
}
```

จากตัวอย่างจะเห็นว่า `NotificationService` (โมดูลระดับสูง) ต้องขึ้นตรงกับ `EmailSender` (โมดูลระดับต่ำ) ทำให้เวลาที่เราอยากเปลี่ยนจากการส่งอีเมลเป็นการส่ง SMS หรือ Push Notification เราจำเป็นต้องแก้ไข `NotificationService` ซึ่งไม่เป็นไปตามหลักการ DIP

ลองนำหลักการ Dependency Inversion มาปรับใช้กันดีกว่า

ขั้นแรกสร้างอินเตอร์เฟส `Notifier` ที่เป็น abstraction ของการแจ้งเตือน โดยไม่สนใจว่าจะส่งด้วยวิธีใด

```go
package notification

import "log"

// Notifier is an abstraction of notification delivery.
type Notifier interface {
        Send(to, message string) error
}
```

จากนั้นให้ `EmailSender` และ `SMSSender` อิมพลิเมนต์อินเตอร์เฟส `Notifier` ซึ่งในภาษา Go นั้นเราไม่จำเป็นต้องประกาศว่าอิมพลิเมนต์อินเตอร์เฟสอะไร เพราะแค่มีฟังก์ชันตามที่อินเตอร์เฟสต้องการก็ถือว่าผ่านแล้ว

```go
type EmailSender struct{}

func (e *EmailSender) Send(to, message string) error {
        log.Printf("sending email to %s: %s", to, message)
        return nil
}

type SMSSender struct{}

func (e *SMSSender) Send(to, message string) error {
        log.Printf("sending SMS to %s: %s", to, message)
        return nil
}
```

และสุดท้ายให้ `NotificationService` รับ `Notifier` ผ่านทางคอนสตรักเตอร์ แทนการสร้างอ็อบเจกต์ขึ้นมาเอง

```go
type NotificationService struct {
        notifier Notifier
}

func NewNotificationService(notifier Notifier) *NotificationService {
        return &NotificationService{notifier: notifier}
}

func (n *NotificationService) Notify(to, message string) error {
        return n.notifier.Send(to, message)
}
```

สังเกตว่าตอนนี้ `NotificationService` ไม่ได้ขึ้นตรงกับ `EmailSender` หรือ `SMSSender` อีกต่อไปแล้ว แต่ทั้งคู่ต่างก็ขึ้นกับอินเตอร์เฟส `Notifier` ที่เป็น abstraction เดียวกัน ทำให้การเปลี่ยนวิธีการส่งแจ้งเตือนทำได้ง่ายขึ้นมาก ต่อให้ในอนาคตอยากเพิ่มช่องทางใหม่อย่าง Push Notification ก็แค่สร้างตัวส่งใหม่ที่อิมพลิเมนต์ `Notifier` เข้าไป โดยที่ไม่ต้องไปยุ่งกับ `NotificationService` เลย

```go
func main() {
        emailNotifier := &EmailSender{}
        smsNotifier := &SMSSender{}

        service := NewNotificationService(emailNotifier)
        service.Notify("user@example.com", "Hello via email!")

        // เปลี่ยนเป็น SMS ได้ทันทีโดยไม่ต้องแก้ไข NotificationService
        service = NewNotificationService(smsNotifier)
        service.Notify("0891234567", "Hello via SMS!")
}
```

---

หลักการ Dependency Inversion ช่วยให้ซอฟต์แวร์ของเรามีความยืดหยุ่นและง่ายต่อการเปลี่ยนแปลง โดยการทำให้โมดูลระดับสูงและระดับต่ำขึ้นอยู่กับ abstraction ร่วมกัน ซึ่งในภาษา Go นั้นอินเตอร์เฟสเป็นเครื่องมือที่ช่วยให้เราทำตามหลักการนี้ได้อย่างเป็นธรรมชาติ