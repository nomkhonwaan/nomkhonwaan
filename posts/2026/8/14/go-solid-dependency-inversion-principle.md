---
title: Go SOLID - Dependency Inversion Principle
publish_date: 2026-08-14
tags: ['go', 'solid']
---

บล็อกนี้เล่าถึงหลักการ Dependency Inversion (DIP) ซึ่งเป็นหลักการสุดท้ายของ SOLID ที่กล่าวไว้โดย Robert C. Martin แปลได้ใจความว่า "โมดูลระดับสูงไม่ควรขึ้นตรงกับโมดูลระดับต่ำ แต่ทั้งคู่ควรขึ้นกับสิ่งที่เป็นนามธรรม (abstraction) และสิ่งที่เป็นนามธรรมไม่ควรขึ้นตรงกับรายละเอียด แต่รายละเอียดควรขึ้นกับสิ่งที่เป็นนามธรรม"

> High-level modules should not depend on low-level modules. Both should depend on abstractions.
> Abstractions should not depend on details. Details should depend on abstractions.

หลายคนอาจจะสงสัยว่าหลักการนี้แตกต่างจาก Inversion of Control (IoC) หรือ Dependency Injection (DI) หรือไม่? ขอตอบว่ามันคนละเรื่องกันครับ DIP คือหลักการ ส่วน DI และ IoC เป็นวิธีการนำหลักการไปใช้งานนั่นเอง

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

ลองนำหลักการ Dependency Inversion มาปรับใช้กัน

ขั้นแรกสร้างอินเตอร์เฟส `Notifier` ที่เป็นนามธรรมของการแจ้งเตือน โดยไม่สนใจว่าจะส่งด้วยวิธีใด

```go
package notification

import "log"

// Notifier is an abstraction of notification delivery.
type Notifier interface {
        Send(to, message string) error
}
```

จากนั้นให้ `EmailSender` และ `SMSSender` อิมพลิเมนต์อินเตอร์เฟส `Notifier`

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

สังเกตว่าตอนนี้ `NotificationService` ไม่ได้ขึ้นตรงกับ `EmailSender` หรือ `SMSSender` อีกต่อไปแล้ว แต่ทั้งคู่ต่างก็ขึ้นกับอินเตอร์เฟส `Notifier` ที่เป็นนามธรรมเดียวกัน ทำให้การเปลี่ยนวิธีการส่งแจ้งเตือนทำได้ง่ายขึ้นมาก

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

หลักการ Dependency Inversion ช่วยให้ซอฟต์แวร์ของเรามีความยืดหยุ่นและง่ายต่อการเปลี่ยนแปลง โดยการทำให้โมดูลระดับสูงและระดับต่ำขึ้นอยู่กับสิ่งที่เป็นนามธรรมร่วมกัน ซึ่งในภาษา Go นั้นอินเตอร์เฟสเป็นเครื่องมือที่ช่วยให้เราทำตามหลักการนี้ได้อย่างเป็นธรรมชาติ