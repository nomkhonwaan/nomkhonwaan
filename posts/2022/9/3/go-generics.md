---
title: Generics ใน Go
publish_date: 2022-09-03
---

Generics เป็นหนึ่งในฟีเจอร์ที่นักพัฒนาเรียกร้องกันมา [ตั้งแต่เปิดตัวภาษา](https://groups.google.com/g/golang-nuts/c/70-pdwUUrbI/m/onMsQspcljcJ?pli=1) และ [เริ่มพัฒนาจริง ๆ ตอนปี 2019](https://go.dev/blog/why-generics) ในที่สุดก็ปล่อยออกมาให้ใช้งานกันแล้วที่เวอร์ชัน 1.18 มาลองดูกันว่าสามารถทำอะไรได้บ้าง

---

Generics เป็นแนวคิดของภาษายุคใหม่ที่อนุญาตให้ฟังก์ชันสามารถทำงานกับตัวแปรประเภทใดก็ได้ที่มีความคล้ายกัน หรือตรงกับเงื่อนไขของฟังก์ชันนั้น ๆ 

ยกตัวอย่างเช่นฟังก์ชัน `sort.Sort` ที่ทำงานได้แค่กับตัวแปรที่อิมพลิเมนต์อินเตอร์เฟซ​ `sort.Interface` เท่านั้นซึ่งถ้าเรียงตัวแปรประเภทอื่น ๆ นอกเหนือไปจาก `sort.Float64Slice`, `sort.IntSlice` และ `sort.StringSlice` จำต้องอิมพลิเมนต์ฟังก์ชันเพิ่มอีก 3 ฟังก์ชันคือ 

- `Len()`
- `Less(i, j int) bool`
- `Swap(i, j int)`

การมาของ generics ช่วยให้การเขียนฟังก์ชันเรียงลำดับเปลี่ยนไป

```go
package main

import (
        "fmt"
        "sort"

        "golang.org/x/exp/constraints"
)

func main() {
        i := []int{6, 5, 1, 3, 4, 2}
        sortAny(i)
        fmt.Println(i)

        f := []float64{4.2, 2.9, 5.32, 1.70, 3.65}
        sortAny(f)
        fmt.Println(f)

        s := []string{"He", "She", "They", "It", "We"}
        sortAny(s)
        fmt.Println(s)
}

func sortAny[T constraints.Ordered](t []T) {
        sort.Slice(t, func(i, j int) bool { return t[i] < t[j] })
}
```

สิ่งที่ต้องมีคือ Go เวอร์ชันตั้งแต่ 1.18 หรือใหม่กว่าน้ัน

---

อ้างอิง

1. [Tutorial: Getting started with generics](https://go.dev/doc/tutorial/generics)

---
#go