---
title: Generics ใน Go
publish_date: 2022-08-18
---

Generics เป็นหนึ่งในฟีเจอร์ที่นักพัฒนาเรียกร้องกันมา [ตั้งแต่เปิดตัวภาษา](https://groups.google.com/g/golang-nuts/c/70-pdwUUrbI/m/onMsQspcljcJ?pli=1) และ [เริ่มพัฒนาจริง ๆ ตอนปี 2019](https://go.dev/blog/why-generics) ในที่สุดก็ปล่อยออกมาให้ใช้งานกันแล้วที่เวอร์ชัน 1.18 มาลองดูกันว่าสามารถทำอะไรได้บ้าง

---

Generics เป็นแนวคิดของภาษายุคใหม่ที่อนุญาตให้ฟังก์ชันสามารถทำงานกับตัวแปรประเภทใดก็ได้ ที่ตรงกับเงื่อนไขของฟังก์ชันนั้น ๆ 

ยกตัวอย่างเช่นฟังก์ชัน `sort.Sort` ที่ทำงานได้แค่กับตัวแปรที่อิมพลิเมนต์อินเตอร์เฟซ​ `sort.Interface` เท่านั้นซึ่งถ้าต้องการเรียงลำดับตัวแปรประเภทอื่นต้องอิมพลิเมนต์ฟังก์ชันเพิ่มอีก 3 ฟังก์ชันคือ 

- `Len()`
- `Less(i, j int) bool`
- `Swap(i, j int)`

การมาของ generics ช่วยให้การเขียนฟังก์ชันเรียงลำดับสามารถส่งสไลด์ของ `[]float64`, `[]int` และ `[]string` เข้าไปที่ฟังก์ชัน `sortAny` ได้ทันทีโดยไม่ต้องแปลงเป็น `sort.Float64Slice`, `sort.IntSlice` และ `sort.StringSlice`

```go
package main

import (
        "fmt"
        "sort"

        "golang.org/x/exp/constraints"
)

func main() {
        f := []float64{4.2, 2.9, 5.32, 1.70, 3.65}
        sortAny(f)
        fmt.Println(f)

        i := []int{6, 5, 1, 3, 4, 2}
        sortAny(i)
        fmt.Println(i)

        s := []string{"He", "She", "They", "It", "We"}
        sortAny(s)
        fmt.Println(s)
}

func sortAny[T constraints.Ordered](t []T) {
        sort.Slice(t, func(i, j int) bool { return t[i] < t[j] })
}
```

สังเกตที่ฟังก์ชัน `sortAny` จะมี syntax ใหม่เป็นวงเล็บสี่เหลี่ยม `[T constraints.Ordered]` มีความหมายว่าตัวแปรประเภทใดที่อยู่อยู่ภายใต้ `constraints.Ordered` สามารถส่งเข้ามาที่ `T` ได้

ถ้าเข้าไปดูอีกนิดจะพบว่า `constraints.Ordered` ประกอบไปด้วย `Integer | Float | ~string`

```go
// Ordered is a constraint that permits any ordered type: any type
// that supports the operators < <= >= >.
// If future releases of Go add new ordered types,
// this constraint will be modified to include them.
type Ordered interface {
	Integer | Float | ~string
}
```

มาดูกันที่ `~string` ความหมายของสัญลักษณ์​ `~` คือนอกจาก `string` แล้วยังรวมไปถึงประเภทของตัวแปรที่ถูกสร้างขึ้นจาก `string` ด้วยเช่น `MyString` ในตัวอย่างด้านล่างนี้
 

```go
func main() {
	...

	u := []MyString{"C", "D", "B", "A", "E"}
	sortAny(u)
	fmt.Println(u)
}

type MyString string
```

---

อ้างอิง

1. [Tutorial: Getting started with generics](https://go.dev/doc/tutorial/generics)

---
#go