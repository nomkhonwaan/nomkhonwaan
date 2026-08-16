---
title: ความแตกต่างของโครงสร้างข้อมูลระหว่าง Array กับ List
publish_date: 2026-08-15
tags: ['data-structure', 'go']
---

เวลาเราเขียนโปรแกรมนอกจากตัวแปรธรรมดาแล้ว เรามักจะต้องเก็บข้อมูลหลาย ๆ ตัวไว้ด้วยกัน ซึ่งวิธีพื้นฐานที่สุดก็คือการเก็บในรูปของ "ชุดข้อมูล" (collection) นั่นเอง

สองโครงสร้างข้อมูลที่พื้นฐานและพบเจอบ่อยที่สุดคือ **Array** และ **List** บทความนี้จะอธิบายความแตกต่างของทั้งสองในเชิงโครงสร้างข้อมูล ก่อนจะพาไปดูว่าในภาษา Go นั้นโครงสร้างข้อมูลแบบ list นั้นถูก implement ไว้ในรูปแบบใด

## TL;DR

- **Array** โครงสร้างข้อมูลขนาดคงที่ (fixed size) เก็บข้อมูลชนิดเดียวกันในตำแหน่งหน่วยความจำที่ติดกัน
- **List** โครงสร้างข้อมูลขนาดปรับเปลี่ยนได้ (dynamic size) สามารถเพิ่มหรือลบสมาชิกได้ตลอด
- ในภาษาส่วนใหญ่ list จะ implement ด้วย dynamic array ที่ขยายขนาดอัตโนมัติ
- **ถ้าคุณใช้ list ในภาษาอื่น (Python, JavaScript, Java) สิ่งนั้นคือ slice ใน Go (`[]T`)**
- ใน Go: array = `[n]T`, list/slice = `[]T`

---

## Array เป็นโครงสร้างข้อมูลที่มีขนาดคงที่

Array คือโครงสร้างข้อมูลที่เก็บชุดของข้อมูลชนิดเดียวกันในตำแหน่งหน่วยความจำที่อยู่ติดกัน (contiguous memory) และที่สำคัญคือมี **ขนาดคงที่ (fixed size)** เมื่อประกาศแล้ว **ไม่สามารถเพิ่มหรือลดขนาด** ได้อีก

```
Address:  [100] [104] [108] [112] [116]
Value:      10    20    30    40    50
Index:       0     1     2     3     4
```

เราสามารถคำนวณหาตำแหน่งของสมาชิกตัวที่ `i` ได้โดยตรงจากสูตร `address[i] = base_address + i * size_of_element` ทำให้การเข้าถึงสมาชิกทำได้ในเวลา **O(1)** เสมอ

### คุณสมบัติของ Array

| Property | Array |
|---|---|
| ขนาด | **Fixed** กำหนดตั้งแต่ประกาศ เปลี่ยนไม่ได้ |
| ชนิดข้อมูล | **Same type** สมาชิกทุกตัวเป็น type เดียวกัน |
| การเข้าถึง | **Random access O(1)** ผ่าน index |
| หน่วยความจำ | **Contiguous** ข้อมูลเรียงติดกัน, cache-friendly |

ตัวอย่าง array ในภาษา C:

```c
int arr[5] = {10, 20, 30, 40, 50};
arr[2] = 99; // random access O(1)
```

ข้อเสียที่สำคัญของ array คือพอขนาดเต็มแล้วไม่สามารถเพิ่มสมาชิกใหม่ได้ ต้องสร้าง array ใหม่ที่ใหญ่กว่าและ copy ข้อมูลเก่าไปแทน

---

## List เป็นโครงสร้างข้อมูลที่ขนาดสามารถปรับเปลี่ยนได้

List คือโครงสร้างข้อมูลที่ **สามารถปรับขนาดได้ (dynamic size)** สามารถเพิ่มหรือลบสมาชิกได้ตลอดเวลา โดยรายละเอียดการจัดการหน่วยความจำด้านหลังถูกซ่อนไว้จากผู้ใช้

ในทางปฏิบัติ list หลาย ๆ ภาษา implement โดยใช้ **dynamic array** (array ที่สามารถขยายขนาดได้เมื่อเต็ม) ซึ่งมีหลักการทำงานดังนี้:

1. จอง array ขนาดหนึ่งไว้ข้างใน (internal array / backing array)
2. เมื่อเพิ่มสมาชิกจนเต็ม ก็สร้าง array ใหม่ที่ใหญ่กว่า (เช่น ขยายเป็น 2 เท่า)
3. copy ข้อมูลเก่าไปยัง array ใหม่
4. ปล่อย array เก่าทิ้ง

กระบวนการนี้เรียกว่า **grow** หรือ **resize** ทำให้เวลาที่เรา append สมาชิกไปเรื่อย ๆ จะมีบางครั้งที่ช้าหน่อย (O(n) ตอน resize) แต่โดยเฉลี่ยแล้ว amortized time อยู่ที่ **O(1)**

#### Amortized Time คืออะไร?

Amortized time หรือ "เวลาแบบเฉลี่ยสะสม" คือการวิเคราะห์ประสิทธิภาพโดยดูจาก **ต้นทุนรวมของการดำเนินการหลายครั้งติดต่อกัน** แล้วหารเฉลี่ยออกมา แทนที่จะดูแค่ครั้งใดครั้งหนึ่ง

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

เมื่อเฉลี่ยต่อครั้ง = 15 / 8 ≈ 1.875 ซึ่งนับว่า **O(1)**

นี่คือที่มาของคำว่า **amortized O(1)** แม้บางครั้งอาจช้า (O(n)) แต่เมื่อมองในภาพรวมของการใช้งานต่อเนื่องหลายครั้งแล้ว ต้นทุนเฉลี่ยต่อครั้งกลับคงที่ เพราะการ resize แต่ละครั้งจะเพิ่มพื้นที่ให้กับ operation ต่อ ๆ ไปอีกหลายครั้ง ทำให้ต้นทุนกระจายตัว อ่านเพิ่มเติมเกี่ยวกับ Amortized Analysis [ได้ที่นี่](https://en.wikipedia.org/wiki/Amortized_analysis)

### คุณสมบัติของ List

| Property | List |
|---|---|
| ขนาด | **Dynamic** เพิ่ม/ลดได้ตลอด |
| ชนิดข้อมูล | **Same type** (ในภาษาที่ type-safe) |
| การเข้าถึง | **Random access O(1)** ผ่าน index (เพราะใช้ dynamic array) |
| การเพิ่มสมาชิก | **O(1) amortized** append, **O(n)** แทรกกลางลำดับ |

---

## List ในภาษาต่าง ๆ

list ในแต่ละภาษาจะมีชื่อเรียกและ syntax ที่แตกต่างกันไป แต่วางตัวเป็นโครงสร้างข้อมูลแบบเดียวกันคือ dynamic size ที่ random access O(1) ได้ และมี internal array อยู่ข้างหลัง:

```python
# Python (list)
nums = [1, 2, 3]
nums.append(4)
nums.append(5)
print(nums)  # [1, 2, 3, 4, 5]
```

```javascript
// JavaScript (Array แตจริง ๆ คือ dynamic array / list)
const nums = [1, 2, 3];
nums.push(4);
nums.push(5);
console.log(nums);  // [1, 2, 3, 4, 5]
```

```java
# Java (ArrayList)
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

ภาษา Go ก็มีทั้งสองโครงสร้างข้อมูลให้ใช้เหมือนกัน โดยเรียก **Array** และ **Slice** ซึ่ง slice ใน Go ก็คือ list ในมุมของ data structure นั่นเอง

### Go Array (ตัวแทนของ Array)

Go Array ตรงกับ concept ของ array ในเชิง data structure (fixed size, same type, contiguous memory):

```go
var arr [5]int   // array ขนาด 5 ตัว เปลี่ยนขนาดไม่ได้
arr[0] = 10
arr[1] = 20
arr[2] = 30
```

Go ไปไกลกว่านั้น **ขนาดของ array เป็นส่วนหนึ่งของ type**:

```go
var a [5]int
var b [10]int
a = b // ❌ compile error: [5]int != [10]int
```

แล้วถ้าลอง `append` สมาชิกเพิ่มเข้าไปใน array ล่ะ? จะเกิดอะไรขึ้น?

```go
arr := [3]int{1, 2, 3}
arr = append(arr, 4) // ❌ compile error: first argument to append must be a slice; have [3]int
```

ข้อความ compile error บอกชัดเจนว่า `append` รับได้เฉพาะ **slice** เท่านั้น ไม่ใช่ array เพราะ array มีขนาดคงที่และไม่สามารถขยายได้

### Go Slice (ตัวแทนของ List)

**Slice** (`[]T`) คือสิ่งที่ตรงกับ list ในเชิง data structure (dynamic size, random access O(1), backed by internal array):

```go
var nums []int              // slice = list ในมุม data structure
nums = append(nums, 10)     // append เหมือน .push() / .add()
nums = append(nums, 20)
nums = append(nums, 30)

fmt.Println(nums)           // [10 20 30]
fmt.Println(nums[1])        // 20 (random access O(1))
```

สังเกตว่าประกาศคล้าย array มาก แต่ **ไม่ระบุขนาดใน `[]`**:

```go
arr := [3]int{1, 2, 3}  // ✅ Array (fixed size)
slc := []int{1, 2, 3}   // ✅ Slice = List (dynamic size)
```

### การทำงานของ Slice เบื้องหลัง

Slice ถูก implement เป็น dynamic array เช่นเดียวกับ list ในภาษาอื่น:

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

- **Pointer** ชี้ไปยัง internal array
- **Length** จำนวนสมาชิกปัจจุบัน
- **Capacity** ขยายได้ถึงเท่าไหร่โดยไม่ต้องจอง array ใหม่

เมื่อ append จน capacity เต็ม Go จะสร้าง array ใหม่ที่ใหญ่กว่า (โดยปกติขยาย 2 เท่า) แล้ว copy ข้อมูลไป เช่นเดียวกับหลักการของ dynamic array ในภาษาอื่น:

```go
nums := []int{1, 2, 3}     // len=3, cap=3
nums = append(nums, 4)      // len=4, cap=6 (ขยายเป็น 2 เท่า)
nums = append(nums, 5, 6)   // len=6, cap=6
nums = append(nums, 7)      // len=7, cap=12 (ขยายอีกครั้ง)
```

---

## สรุป

- **Array** คือโครงสร้างข้อมูลแบบ **fixed size** ใน Go ใช้ `[n]T`
- **List** คือโครงสร้างข้อมูลแบบ **dynamic size** ใน Go ใช้ `[]T` (slice)

ในทางปฏิบัติ Go community แนะนำให้ใช้ **slice (`[]T`)** เป็นค่าเริ่มต้นในการเขียนโค้ด ส่วน array (`[n]T`) มีไว้สำหรับกรณีเฉพาะเท่านั้น เช่น การประกาศขนาดตายตัวที่ต้องการประสิทธิภาพสูง

การเข้าใจ concept ของโครงสร้างข้อมูลให้ถูกต้องเป็นสิ่งสำคัญ เมื่อเราเข้าใจแล้วว่า array คืออะไร list คืออะไร การย้ายไปใช้ภาษาไหนก็ทำได้ไม่ยาก เพียงแค่รู้ว่าภาษานั้นเรียกมันว่าอะไรและใช้ syntax แบบไหน
