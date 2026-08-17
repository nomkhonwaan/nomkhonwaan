---
title: ความแตกต่างของโครงสร้างข้อมูลระหว่าง Queue กับ Stack
publish_date: 2026-08-17
tags: ['data-structure', 'go']
---

เวลาเราต่อคิวซื้อของหรือซ้อนจานที่บ้าน เรากำลังใช้โครงสร้างข้อมูลสองอย่างโดยไม่รู้ตัว นั่นก็คือ คิว (Queue) และ สแต็ก (Stack)

สองโครงสร้างข้อมูลนี้เป็นพื้นฐานที่สำคัญมากในการเขียนโปรแกรม ตั้งแต่การจัดการงานในระบบปฏิบัติการ ไปจนถึงการทำงานของเบราว์เซอร์ บทความนี้จะพาไปดูว่า queue และ stack คืออะไร ทำงานอย่างไร และมีตัวอย่างการใช้งานในโลกจริงพร้อมโค้ดภาษา Go

## TL;DR

- Queue: โครงสร้างข้อมูลแบบ FIFO (First In, First Out) ตัวแรกที่เข้า คือตัวแรกที่ออก
- Stack: โครงสร้างข้อมูลแบบ LIFO (Last In, First Out) ตัวสุดท้ายที่เข้า คือตัวแรกที่ออก
- Queue เหมาะกับงานที่ต้องประมวลผลตามลำดับคอย เช่น print spooler, task scheduler
- Stack เหมาะกับงานที่ต้องย้อนกลับหรือ undo เช่น browser history, function call stack
- Go ไม่มี queue/stack ใน standard library โดยตรง แต่สามารถสร้างเองได้ตามหลักการ

---

## Queue: โครงสร้างข้อมูลแบบ FIFO

Queue หรือ คิว คือโครงสร้างข้อมูลที่ทำงานแบบ FIFO (First In, First Out) หมายความว่า ข้อมูลที่เข้ามาก่อนจะถูกนำออกก่อน เหมือนกับการต่อคิวซื้อของตามร้านสะดวกซื้อ คนที่มายืนต่อคิวก่อนก็จะได้ซื้อของก่อน

```
    In → [ rear ] [ ... ] [ ... ] [ front ] → Out
         (enqueue)                (dequeue)
```

### การทำงานพื้นฐานของ Queue

Queue มี operation หลัก ๆ ดังนี้:

| Operation | คำอธิบาย | Time Complexity |
|---|---|---|
| `enqueue(x)` | เพิ่มข้อมูลต่อท้ายคิว | O(1) |
| `dequeue()` | นำข้อมูลหน้าคิวออก | O(1) |
| `front()` / `peek()` | ดูข้อมูลหน้าคิวโดยไม่นำออก | O(1) |
| `isEmpty()` | ตรวจสอบว่าคิวว่างหรือไม่ | O(1) |
| `size()` | ดูจำนวนสมาชิกในคิว | O(1) |

### ตัวอย่าง Queue ในชีวิตจริง

- การต่อคิวในชีวิตประจำวัน: คนที่มายืนต่อคิวก่อนได้บริการก่อน
- Print spooler: งานพิมพ์ที่ส่งมาก่อนจะถูกพิมพ์ก่อน
- Task scheduler: งานที่ถูกส่งเข้ามาก่อนจะถูกประมวลผลก่อน
- Message queue: RabbitMQ, Kafka ใช้ queue เพื่อส่งข้อความระหว่าง service
- Breadth-First Search (BFS): ใช้วนลูปเพื่อค้นหาข้อมูลในกราฟแบบกว้าง

---

## Stack: โครงสร้างข้อมูลแบบ LIFO

Stack หรือ สแต็ก คือโครงสร้างข้อมูลที่ทำงานแบบ LIFO (Last In, First Out) หมายความว่า ข้อมูลที่เข้ามาทีหลังจะถูกนำออกก่อน เหมือนกับการซ้อนจานหรือซ้อนหนังสือ จานที่วางซ้อนทีหลังจะอยู่บนสุดและถูกหยิบออกก่อน

```
      Out ← [ top ]
            [ ... ]
            [ ... ]
            [ ... ]
    In → [ bottom ]
```

### การทำงานพื้นฐานของ Stack

Stack มี operation หลัก ๆ ดังนี้:

| Operation | คำอธิบาย | Time Complexity |
|---|---|---|
| `push(x)` | เพิ่มข้อมูลลงบนสุดของสแต็ก | O(1) |
| `pop()` | นำข้อมูลบนสุดออก | O(1) |
| `top()` / `peek()` | ดูข้อมูลบนสุดโดยไม่นำออก | O(1) |
| `isEmpty()` | ตรวจสอบว่าสแต็กว่างหรือไม่ | O(1) |
| `size()` | ดูจำนวนสมาชิกในสแต็ก | O(1) |

### ตัวอย่าง Stack ในชีวิตจริง

- การซ้อนจาน: จานที่วางซ้อนทีหลังอยู่บนสุดและถูกหยิบออกก่อน
- Undo / Redo: การย้อนกลับการทำงานในโปรแกรมแก้ไขเอกสาร
- Browser history: ปุ่ม Back จะพาเรากลับไปยังหน้าที่เพิ่งดูมาก่อน
- Function call stack: ภาษาโปรแกรมทุกภาษาใช้ stack ในการเรียกฟังก์ชัน
- Depth-First Search (DFS): ใช้วนลูปเพื่อค้นหาข้อมูลในกราฟแบบลึก
- Expression evaluation: ใช้ stack ในการคำนวณนิพจน์ทางคณิตศาสตร์

---

## Queue และ Stack ในภาษา Go

ภาษา Go ไม่ได้มี queue หรือ stack เป็นโครงสร้างข้อมูลในตัว (built-in) แต่เราสามารถ implement ได้ง่าย ๆ โดยใช้ slice หรือ container/list

### Queue ด้วย Slice

```go
type Queue[T any] struct {
	items []T
}

func (q *Queue[T]) Enqueue(item T) {
	q.items = append(q.items, item)
}

func (q *Queue[T]) Dequeue() (T, bool) {
	if q.IsEmpty() {
		var zero T
		return zero, false
	}
	item := q.items[0]
	q.items = q.items[1:]
	return item, true
}

func (q *Queue[T]) Front() (T, bool) {
	if q.IsEmpty() {
		var zero T
		return zero, false
	}
	return q.items[0], true
}

func (q *Queue[T]) IsEmpty() bool {
	return len(q.items) == 0
}

func (q *Queue[T]) Size() int {
	return len(q.items)
}
```

ข้อควรระวัง: การ `Dequeue` โดยใช้ `q.items = q.items[1:]` จะไม่คืนหน่วยความจำส่วนที่ถูกตัดทิ้งไป ถ้าต้องการประสิทธิภาพสูง อาจใช้ circular queue หรือ linked list แทน

### Stack ด้วย Slice

```go
type Stack[T any] struct {
	items []T
}

func (s *Stack[T]) Push(item T) {
	s.items = append(s.items, item)
}

func (s *Stack[T]) Pop() (T, bool) {
	if s.IsEmpty() {
		var zero T
		return zero, false
	}
	item := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return item, true
}

func (s *Stack[T]) Top() (T, bool) {
	if s.IsEmpty() {
		var zero T
		return zero, false
	}
	return s.items[len(s.items)-1], true
}

func (s *Stack[T]) IsEmpty() bool {
	return len(s.items) == 0
}

func (s *Stack[T]) Size() int {
	return len(s.items)
}
```

### Queue ด้วย container/list (Linked List)

Go มี `container/list` ซึ่งเป็น doubly linked list ที่สามารถใช้ implement queue ได้:

```go
import "container/list"

type QueueList[T any] struct {
	items *list.List
}

func NewQueueList[T any]() *QueueList[T] {
	return &QueueList[T]{items: list.New()}
}

func (q *QueueList[T]) Enqueue(item T) {
	q.items.PushBack(item)
}

func (q *QueueList[T]) Dequeue() (T, bool) {
	if q.IsEmpty() {
		var zero T
		return zero, false
	}
	front := q.items.Front()
	q.items.Remove(front)
	return front.Value.(T), true
}

func (q *QueueList[T]) IsEmpty() bool {
	return q.items.Len() == 0
}
```

### Stack ด้วย container/list

```go
import "container/list"

type StackList[T any] struct {
	items *list.List
}

func NewStackList[T any]() *StackList[T] {
	return &StackList[T]{items: list.New()}
}

func (s *StackList[T]) Push(item T) {
	s.items.PushFront(item)
}

func (s *StackList[T]) Pop() (T, bool) {
	if s.IsEmpty() {
		var zero T
		return zero, false
	}
	front := s.items.Front()
	s.items.Remove(front)
	return front.Value.(T), true
}

func (s *StackList[T]) IsEmpty() bool {
	return s.items.Len() == 0
}
```

---

## ตัวอย่างการใช้งานจริง

### ตัวอย่างที่ 1: Print Spooler (Queue)

จำลองระบบส่งงานพิมพ์ที่ต้องพิมพ์ตามลำดับที่ส่งเข้ามา:

```go
package main

import (
	"fmt"
	"time"
)

type PrintJob struct {
	ID       int
	Filename string
}

type PrintSpooler struct {
	queue Queue[PrintJob]
}

func (ps *PrintSpooler) Submit(job PrintJob) {
	ps.queue.Enqueue(job)
	fmt.Printf("Job #%d (%s) submitted\n", job.ID, job.Filename)
}

func (ps *PrintSpooler) Process() {
	for !ps.queue.IsEmpty() {
		job, _ := ps.queue.Dequeue()
		fmt.Printf("Printing: #%d (%s)...\n", job.ID, job.Filename)
		time.Sleep(500 * time.Millisecond)
		fmt.Printf("Done: #%d (%s)\n", job.ID, job.Filename)
	}
}

func main() {
	spooler := &PrintSpooler{}

	spooler.Submit(PrintJob{1, "report.pdf"})
	spooler.Submit(PrintJob{2, "invoice.pdf"})
	spooler.Submit(PrintJob{3, "photo.jpg"})

	fmt.Println("\n--- Start Printing ---")
	spooler.Process()
}
```

ผลลัพธ์:

```
Job #1 (report.pdf) submitted
Job #2 (invoice.pdf) submitted
Job #3 (photo.jpg) submitted

--- Start Printing ---
Printing: #1 (report.pdf)...
Done: #1 (report.pdf)
Printing: #2 (invoice.pdf)...
Done: #2 (invoice.pdf)
Printing: #3 (photo.jpg)...
Done: #3 (photo.jpg)
```

จะเห็นว่างานพิมพ์ที่ส่งเข้ามาก่อน (report.pdf) ถูกพิมพ์ก่อนเสมอ ตามหลัก FIFO

### ตัวอย่างที่ 2: Browser History (Stack)

จำลองการทำงานของปุ่ม Back ในเบราว์เซอร์:

```go
package main

import "fmt"

type BrowserHistory struct {
	backStack    Stack[string]
	forwardStack Stack[string]
	current      string
}

func NewBrowserHistory(homepage string) *BrowserHistory {
	return &BrowserHistory{
		current: homepage,
	}
}

func (b *BrowserHistory) Visit(url string) {
	b.backStack.Push(b.current)
	b.current = url
	// clear forward history on new visit
	b.forwardStack = Stack[string]{}
	fmt.Printf("Visiting: %s\n", b.current)
}

func (b *BrowserHistory) Back() {
	if page, ok := b.backStack.Pop(); ok {
		b.forwardStack.Push(b.current)
		b.current = page
		fmt.Printf("Back to: %s\n", b.current)
	} else {
		fmt.Println("No back history")
	}
}

func (b *BrowserHistory) Forward() {
	if page, ok := b.forwardStack.Pop(); ok {
		b.backStack.Push(b.current)
		b.current = page
		fmt.Printf("Forward to: %s\n", b.current)
	} else {
		fmt.Println("No forward history")
	}
}

func main() {
	browser := NewBrowserHistory("google.com")

	browser.Visit("github.com")
	browser.Visit("medium.com")
	browser.Visit("youtube.com")

	browser.Back()  // back to medium.com
	browser.Back()  // back to github.com
	browser.Forward() // forward to medium.com
	browser.Visit("x.com") // clear forward history on new visit

	browser.Back() // back to medium.com
}
```

ผลลัพธ์:

```
Visiting: github.com
Visiting: medium.com
Visiting: youtube.com
Back to: medium.com
Back to: github.com
Forward to: medium.com
Visiting: x.com
Back to: medium.com
```

จะเห็นว่า stack ช่วยให้เราสามารถย้อนกลับและไปข้างหน้าได้อย่างอิสระ โดยใช้หลักการ LIFO นั่นเอง

---

## สรุป

Queue และ Stack เป็นโครงสร้างข้อมูลพื้นฐานที่เรียกว่า linear data structure เหมือนกัน แต่แตกต่างกันที่กฎในการเข้า-ออก:

- Queue: FIFO: มาก่อนออกก่อน เหมาะกับงานที่ต้องรักษาลำดับ
- Stack: LIFO: มาทีหลังออกก่อน เหมาะกับงานที่ต้องย้อนกลับ

ในภาษา Go เราสามารถ implement ทั้งสองโครงสร้างนี้ได้ง่าย ๆ ด้วย slice หรือ `container/list` โดย slice จะมีประสิทธิภาพดีกว่าในกรณีทั่วไป แต่ `container/list` จะเหมาะกับ queue ที่ต้อง enqueue/dequeue บ่อยครั้งเพราะไม่ต้องเลื่อนสมาชิก

การเลือกใช้ queue หรือ stack ขึ้นอยู่กับโจทย์ที่เราเจอ ถ้าต้องการความเป็นธรรมและรักษาลำดับ ใช้ queue ถ้าต้องการย้อนกลับหรือจัดการข้อมูลที่ซ้อนกัน ใช้ stack ครับ