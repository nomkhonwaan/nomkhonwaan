---
title: ความแตกต่างของโครงสร้างข้อมูลระหว่าง Array กับ List
publish_date: 2026-08-15
tags: ['data-structure', 'go']
---

เวลาเราเขียนโปรแกรมนอกจากตัวแปรธรรมดาแล้ว เรามักจะต้องเก็บข้อมูลหลาย ๆ ตัวไว้ด้วยกัน ซึ่งวิธีพื้นฐานที่สุดก็คือการเก็บในรูปของ "ชุดข้อมูล" หรือ collection นั่นเอง

สองโครงสร้างข้อมูลที่พื้นฐานและพบเจอบ่อยที่สุดก็คือ Array กับ List บทความนี้จะพาไปดูความแตกต่างของทั้งสองในเชิงโครงสร้างข้อมูล ก่อนจะมาเจาะลึกกันว่าในภาษา Go นั้น list ถูก implement ไว้ในรูปแบบไหนบ้าง

## TL;DR

- Array โครงสร้างข้อมูลขนาดคงที่ (fixed size) เก็บข้อมูลชนิดเดียวกันในตำแหน่งหน่วยความจำที่ติดกัน
- List โครงสร้างข้อมูลขนาดปรับเปลี่ยนได้ (dynamic size) สามารถเพิ่มหรือลบสมาชิกได้ตลอด
- ในภาษาส่วนใหญ่ list จะ implement ด้วย dynamic array ที่ขยายขนาดอัตโนมัติ
- ใน Go: array = `[n]T`, list/slice = `[]T`

---

## Array เป็นโครงสร้างข้อมูลที่มีขนาดคงที่

Array คือโครงสร้างข้อมูลที่เก็บชุดของข้อมูลชนิดเดียวกันในตำแหน่งหน่วยความจำที่อยู่ติดกัน หรือที่เรียกว่า contiguous memory และที่สำคัญคือมี ขนาดคงที่ (fixed size) เมื่อประกาศแล้ว ไม่สามารถเพิ่มหรือลดขนาด ได้อีก

```
Address:  [100] [104] [108] [112] [116]
Value:      10    20    30    40    50
Index:       0     1     2     3     4
```

ข้อดีของ array คือเราสามารถคำนวณหาตำแหน่งของสมาชิกตัวที่ `i` ได้โดยตรงจากสูตร `address[i] = base_address + i * size_of_element` ทำให้การเข้าถึงสมาชิกทำได้ในเวลา O(1) เสมอ ไม่ว่าอาเรย์จะยาวแค่ไหนก็ตาม

### คุณสมบัติของ Array

| Property | Array |
|---|---|
| ขนาด | Fixed กำหนดตั้งแต่ประกาศ เปลี่ยนไม่ได้ |
| ชนิดข้อมูล | Same type สมาชิกทุกตัวเป็น type เดียวกัน |
| การเข้าถึง | Random access O(1) ผ่าน index |
| หน่วยความจำ | Contiguous ข้อมูลเรียงติดกัน เป็นมิตรกับ CPU cache |

ตัวอย่าง array ในภาษา C:

```c
int arr[5] = {10, 20, 30, 40, 50};
arr[2] = 99; // random access O(1)
```

ข้อเสียที่สำคัญของ array ก็คือพอขนาดเต็มแล้วไม่สามารถเพิ่มสมาชิกใหม่ได้ ต้องสร้าง array ใหม่ที่ใหญ่กว่าแล้ว copy ข้อมูลเก่าไปแทน ซึ่งถ้าต้องทำบ่อย ๆ ก็จะเสียเวลาเปล่า ๆ

---

## List เป็นโครงสร้างข้อมูลที่ขนาดปรับเปลี่ยนได้

List เป็นโครงสร้างข้อมูลที่ ปรับขนาดได้ (dynamic size) เราสามารถเพิ่มหรือลบสมาชิกได้ตลอดเวลา โดยรายละเอียดการจัดการหน่วยความจำด้านหลังถูกซ่อนไว้จากผู้ใช้

ในทางปฏิบัติ list หลาย ๆ ภาษา implement โดยใช้ dynamic array ซึ่งก็คือ array ที่ขยายขนาดได้เมื่อเต็ม โดยมีหลักการทำงานประมาณนี้ครับ:

1. จอง array ขนาดหนึ่งไว้ข้างใน (internal array หรือ backing array)
2. เมื่อเพิ่มสมาชิกจนเต็ม ก็สร้าง array ใหม่ที่ใหญ่กว่า (เช่น ขยายเป็น 2 เท่า)
3. copy ข้อมูลเก่าไปยัง array ใหม่
4. ปล่อย array เก่าทิ้ง

กระบวนการนี้เรียกว่า grow หรือ resize ซึ่งถึงแม้เวลาที่เกิด resize จะช้าหน่อย O(n) แต่ในภาพรวมแล้ว amortized time หรือเวลาเฉลี่ยต่อครั้งกลับอยู่ที่ O(1)

#### Amortized Time คืออะไร?

Amortized time หรือ "เวลาแบบเฉลี่ยสะสม" คือการวัดประสิทธิภาพโดยดูจาก ต้นทุนรวมของการดำเนินการหลายครั้งติดต่อกัน แล้วหารเฉลี่ยออกมา แทนที่จะดูแค่ครั้งใดครั้งหนึ่ง

ยกตัวอย่างการ append สมาชิก 8 ตัวลงใน list ที่เริ่มต้นด้วย capacity = 1 และขยายเป็น 2 เท่าทุกครั้ง:

```
ครั้งที่ 1: append(1)  → cost = 1 (ยังมีที่ว่าง)
ครั้งที่ 2: append(2)  → cost = 2 (resize + copy 1 + insert 1)
ครั้งที่ 3: append(3)  → cost = 3 (resize + copy 2 + insert 1)
ครั้งที่ 4: append(4)  → cost = 1 (ยังมีที่ว่าง)
ครั้งที่ 5: append(5)  → cost = 5 (resize + copy 4 + insert 1)
ครั้งที่ 6: append(6)  → cost = 1
ครั้งที่ 7: append(7)  → cost = 1
ครั้งที่ 8: append(8)  → cost = 1
```

รวมต้นทุนทั้งหมด = 1 + 2 + 3 + 1 + 5 + 1 + 1 + 1 = 15

เมื่อเฉลี่ยต่อครั้ง = 15 / 8 ≈ 1.875 ซึ่งนับว่า O(1)

นี่คือที่มาของคำว่า amortized O(1) ถึงแม้บางครั้งจะช้า (O(n)) แต่เมื่อมองในภาพรวมของการใช้งานต่อเนื่องหลายครั้งแล้ว ต้นทุนเฉลี่ยต่อครั้งกลับคงที่ เพราะการ resize แต่ละครั้งจะเพิ่มพื้นที่ให้กับ operation ต่อ ๆ ไปอีกหลายครั้ง ทำให้ต้นทุนกระจายตัวออกไปนั่นเอง

อ่านเพิ่มเติมเกี่ยวกับ Amortized Analysis [ได้ที่นี่](https://en.wikipedia.org/wiki/Amortized_analysis)

### คุณสมบัติของ List

| Property | List |
|---|---|
| ขนาด | Dynamic เพิ่ม/ลดได้ตลอด |
| ชนิดข้อมูล | Same type (ในภาษาที่ type-safe) |
| การเข้าถึง | Random access O(1) ผ่าน index (เพราะใช้ dynamic array) |
| การเพิ่มสมาชิก | O(1) amortized ตอนท้าย, O(n) ถ้าแทรกกลาง |

---

## List ในภาษาต่าง ๆ

list ในแต่ละภาษาจะมีชื่อเรียกแตกต่างกันไป แต่วางตัวเป็นโครงสร้างข้อมูลแบบเดียวกันคือปรับขนาดได้, random access O(1), และมี internal array อยู่ข้างหลัง:

```python
# Python (list)
nums = [1, 2, 3]
nums.append(4)
nums.append(5)
print(nums)  # [1, 2, 3, 4, 5]
```

```javascript
// JavaScript (Array is actually a dynamic array / list)
const nums = [1, 2, 3];
nums.push(4);
nums.push(5);
console.log(nums);  // [1, 2, 3, 4, 5]
```

```java
// Java (ArrayList)
List<Integer> nums = new ArrayList<>();
nums.add(1);
nums.add(2);
nums.add(3);
nums.add(4);
System.out.println(nums);  // [1, 2, 3, 4]
```

จะเห็นว่าทุกภาษามี list ที่สามารถเพิ่มสมาชิกได้เรื่อย ๆ โดยผู้ใช้ไม่ต้องสนใจการจัดการหน่วยความจำด้านหลัง

---

## แล้วในภาษา Go ล่ะ?

ภาษา Go ก็มีทั้งสองโครงสร้างข้อมูลให้ใช้เหมือนกัน โดยเรียก Array และ Slice ซึ่ง slice ใน Go ก็คือ list ในมุมของ data structure นั่นเอง

### Go Array (ตัวแทนของ Array)

Go Array ตรงกับ concept ของ array ในเชิง data structure คือ fixed size, same type, contiguous memory:

```go
var arr [5]int   // fixed-size array, cannot be resized
arr[0] = 10
arr[1] = 20
arr[2] = 30
```

แต่สิ่งที่ทำให้ Go น่าสนใจคือ ขนาดของ array เป็นส่วนหนึ่งของ type ซึ่งหมายความว่า `[5]int` กับ `[10]int` คือ คนละประเภทกัน:

```go
var a [5]int
var b [10]int
a = b // compile error: [5]int != [10]int
```

แล้วถ้าลอง `append` สมาชิกเพิ่มเข้าไปใน array ล่ะ? จะเกิดอะไรขึ้น?

```go
arr := [3]int{1, 2, 3}
arr = append(arr, 4) // compile error: first argument to append must be a slice; have [3]int
```

compile error บอกชัดเจนครับว่า `append` ใช้กับ slice เท่านั้น ไม่ใช่ array เพราะ array มีขนาดคงที่นั่นเอง

### Go Slice (ตัวแทนของ List)

Slice (`[]T`) คือสิ่งที่ตรงกับ list ในเชิง data structure คือ dynamic size, random access O(1), backed by internal array:

```go
var nums []int              // slice = list in data structure terms
nums = append(nums, 10)     // append like .push() / .add()
nums = append(nums, 20)
nums = append(nums, 30)

fmt.Println(nums)           // [10 20 30]
fmt.Println(nums[1])        // 20 (random access O(1))
```

สังเกตว่าประกาศคล้ายกับ array มาก แต่ ไม่ระบุขนาดใน `[]`:

```go
arr := [3]int{1, 2, 3}  // Array (fixed size)
slc := []int{1, 2, 3}   // Slice = List (dynamic size)
```

### การทำงานของ Slice เบื้องหลัง

Slice ใน Go ถูก implement เป็น dynamic array เช่นเดียวกับ list ในภาษาอื่นครับ:

```go
nums := make([]int, 3, 5) // type []int, length=3, capacity=5
```

```
Slice header:
+---------+----------+----------+
| Pointer | Length=3 | Cap=5    |
+----+----+----------+----------+
     |
     v  (underlying array)
  +---+---+---+---+---+
  | 0 | 0 | 0 |   |   |
  +---+---+---+---+---+
```

- Pointer ชี้ไปยัง internal array
- Length จำนวนสมาชิกปัจจุบัน
- Capacity ขยายได้ถึงเท่าไหร่โดยไม่ต้องจอง array ใหม่

เมื่อ append จน capacity เต็ม Go จะสร้าง array ใหม่ที่ใหญ่กว่า (โดยปกติขยาย 2 เท่า) แล้ว copy ข้อมูลไป เหมือนกับหลักการของ dynamic array ในภาษาอื่นทุกประการ:

```go
nums := []int{1, 2, 3}      // len=3, cap=3
nums = append(nums, 4)      // len=4, cap=6 (doubles)
nums = append(nums, 5, 6)   // len=6, cap=6
nums = append(nums, 7)      // len=7, cap=12 (doubles again)
```

---

## สรุป

Array และ List เป็นโครงสร้างข้อมูลพื้นฐานที่ใช้เก็บชุดข้อมูลเหมือนกัน แต่แตกต่างกันที่การจัดการขนาด:

- **Array**: Fixed size (`[n]T`) — ประกาศแล้วเปลี่ยนขนาดไม่ได้
- **List**: Dynamic size (`[]T`) — เพิ่ม/ลดสมาชิกได้ตลอด

ในทางปฏิบัติ Go community แนะนำให้ใช้ **slice (`[]T`)** เป็นค่าเริ่มต้น ส่วน **array (`[n]T`)** มีไว้สำหรับกรณีเฉพาะ เช่น ค่าคงที่ชุดเล็ก ๆ หรือต้องการควบคุม memory layout

การเข้าใจ concept ของโครงสร้างข้อมูลให้ถูกต้องเป็นเรื่องสำคัญ เมื่อเราเข้าใจแล้วว่า array คืออะไร list คืออะไร การย้ายไปใช้ภาษาไหนก็ทำได้ไม่ยาก แค่รู้ว่าภาษานั้นเรียกมันว่าอะไรและใช้ syntax แบบไหนก็พอครับ
