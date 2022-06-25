---
title: Go SOLID - Single Responsiblity Principle
publish_date:  2020-01-10
---

บล็อกนี้เล่าถึงหลักการ Single Responsibility Principle (SRP) หนึ่งในหลักการของ SOLID ที่กล่าวไว้โดย Robert C. Martin หรือ Uncle Bob ในหนังสือเรื่อง [Agile Software Development, Principles, Patterns, and Practices](https://en.wikipedia.org/wiki/Single_responsibility_principle#cite_note-cleancode-1) สรุปใจความสั้น ๆ ได้ว่า "คลาสควรจะมีเพียงเหตุผลเดียวที่ทำให้ต้องแก้ไข"

> A class should have only one reason to change

แต่ทว่าประโยคนี้ดันสร้างความสับสนให้กับคนทั่วไป Uncle Bob เลยมาขยายความคำว่า "เหตุผล" ในบล็อก [The Single Responsibility Principle](https://blog.cleancoder.com/uncle-bob/2014/05/08/SingleReponsibilityPrinciple.html) อีกที

> The principle is about people (actor)

ลองแปลเป็นไทยแบบสรุปง่าย ๆ ได้ว่า "เหตุผลที่ทำให้จำเป็นต้องแก้ไขโปรแกรมคือคน" อ่านแล้วก็ยังไม่เข้าใจอยู่ดี!? งั้นลองมาดูคำอธิบายเพิ่มเติมพร้อมกับตัวอย่างโค้ดกันดีกว่า

## สารบัญ

- Go SOLID - Single Responsibility Principle
- [Go SOLID - Open-closed Principle](/2020/1/2/go-solid-open-closed-principle)
- [Go SOLID - Interface Segregation Principle](/2019/12/20/go-solid-interface-segregation-principle)

---

ในบทความของ Uncle Bob ได้ยกตัวอย่างโปรแกรม Java โดยมีคลาสชื่อว่า `Employee` และฟังก์ชันสำหรับทำงานตามนี้

```java
public class Employee {
    public Money() calculatePay();
    public void save();
    public String reportHours();
}
```

สมมติว่าแต่ละฟังก์ชันจะมีเจ้าของเป็นระดับ C-Level แบ่งกันตามนี้ ซึ่งถ้าฟังก์ชันไหนทำงานผิดพลาดก็อาจจะส่งผลให้ผู้ดูแลต้องถูกไล่ออกได้

- `calculatePay()` ดูแลโดย Chief Financial Officer (CFO)
- `save()` ดูแลโดย Chief Technology Officer (CTO)
- `reportHours()` ดูแลโดย Chief Operating Officer (COO)

ทีนี้ถ้า COO ต้องการให้คุณแก้ไขฟังก์ชัน `reportHours` แต่บังเอิญว่าคุณดันลืมแก้ไขโค้ดส่วนที่เกี่ยวข้องกับฟังก์ชัน `calculatePay` ทำให้การคำนวณค่าแรงผิดพลาด และนั่นทำให้ CFO ต้องถูกไล่ออก ทั้ง ๆ ที่เขาไม่ได้เป็นคนมอบหมายให้คุณแก้โค้ดตรงส่วนนี้เลย จากบทเรียนนี้ท้ายที่สุดแล้วทุกการแก้ไขไม่ว่าจะส่วนใดของโปรแกรม คุณจำเป็นจะต้องได้รับความยินยอมจากทุก C-Level ก่อนเสมอ

หรืออีกหนึ่งตัวอย่างในโลกความเป็นจริง สมมติคุณขับรถเข้าไปที่ศูนย์เพื่อให้ช่างเทคนิคซ่อมหน้าต่างไฟฟ้าที่มันเปิดไม่ได้ วันถัดมาคุณได้รับโทรศัพท์แจ้งว่ารถของคุณซ่อมเสร็จแล้วสามารถเข้ามารับได้ คุณไปถึงที่ศูนย์และตรวจสอบความเรียบร้อยของหน้าต่างไฟฟ้าและมันก็ทำงานได้ยอดเยี่ยม! แต่เมื่อคุณขึ้นมานั่งหลังพวงมาลัยและกำลังจะสตาร์ทรถเพื่อขับกลับบ้าน ปรากฏว่ามันสตาร์ทไม่ติด ช่างอาจจะลืมใส่สายไฟกลับเข้าที่เดิมหรือแบตเตอรี่รถของคุณอาจจะหมดเกลี้ยงระหว่างการทดสอบ และนั่นไม่ใช่สิ่งที่คุณคาดหวัง

เช่นกันกับในโลกของการพัฒนาโปรแกรม เมื่อมีการแก้ไขใด ๆ ทุกคนย่อมคาดหวังว่ามันจะต้องไม่กระทบกับระบบอื่น ๆ ด้วยหรืออย่างน้อยที่สุดก็ควรจะเป็นฟังก์ชันที่เกี่ยวข้องกัน ดังนั้นแล้วสิ่งที่ต้องคำนึงเสมอเวลาออกแบบโปรแกรมคือทำอย่างไรให้การแก้ไขในแต่ละชิ้นส่วนต้องไม่กระทบกับส่วนอื่น ๆ ของโปรแกรมที่ไม่เกี่ยวข้องได้นั่นเอง

> When you write a software module, you want to make sure that when changes are requested, those changes can only originate from a single person, or rather, a single tightly coupled group of people representing a single narrowly defined business function.

---

ลองเอาตัวอย่างโค้ดข้างต้นมาเขียนด้วย Go

```go
type Money float64

type Employee struct {
        salaryPerHour Money
        workingHours  int
}

// CalculatePay calculates a total amount to-be paid to the employee.
func (e Employee) CalculatePay() Money {
	return e.salaryPerHour * Money(e.workingHours)
}

// Save persits an employee data to the database.
func (e Employee) Save() error {
        ...
}

// ReportHours returns number of working hours in a day.
func (e Employee) ReportHours() string {
        ...
}
```

ตอนนี้ทั้งสามฟังก์ชันยังคงถูกต้องตามหลัก SRP อยู่ ลองเพิ่มเงื่อนไขสำหรับพนักงานที่จ้างเป็นรายเดือนหรือรายปี

```go
type Employee struct {
        ...
        // An employee contract type which are daily, monthly, annually, etc.
        contractType string
}

func (e Employee) CalculatePay() Money {
        switch e.contractType {
        case "daily":
                return e.salaryPerHour * Money(e.workingHours)
        case "monthly":
                return e.salaryPerHour * 24 * 30 * Money(e.workingHours)
        ...
        }
}
```

จะพบว่าต้องมีการแก้ไขฟังก์ชัน `CalculatePay` ทุก ๆ ครั้งที่มีการเพิ่มเงื่อนไขเข้าไป แบบนี้ไม่ดีแน่ลองมาปรับปรุงตามหลัก SRP ดีกว่า

เริ่มจากยกฟังก์ชัน `Save` ออกจาก `Employee` และให้ `EmployeeDAO` รับหน้าที่ดูแลการติดต่อสื่อสารกับดาต้าเบสไป

```go
// EmployeeDAO uses for accessing the database.
type EmployeeDAO struct { ... }

// Save accepts an employee interface which providing functions
// for saving employee to the database.
func (dao EmployeeDAO) Save(emp Employee) error {
        ...
}
```

จากนั้นสร้างอินเตอร์เฟส `Employee` ขึ้นมาเพื่อที่จะแยกประเภทพนักงานแต่ละประเภทออกจากกัน โดยที่แต่ละประเภทก็จะอิมพลิเมนต์ตามอินเตอร์เฟส `Employee`

```go
type Employee interface {
        // Return an amount to-be paid per working hour
        SalaryPerHour() Money
        // Return total number of working hour of the employee
        TotalWorkingHours() int
        ...
}

// DailyEmployee pays daily.
type DailyEmployee struct { ... }

// MonthlyEmployee pays monthly.
type MonthlyEmployee struct { ... }

// ContractEmployee pays monthly until the end of the contract.
type ContractEmployee struct {
        // A begin date of the contract
        beginContractDate time.Time 
        // An ending date of the contract
        endContractDate time.Time
}

func (emp ContractEmployee) SalaryPerHour() Money {
        ...
}

func (emp ContractEmployee) TotalWorkingHours() int {
        return int(emp.endContractDate.Sub(emp.beginContractDate).Hours())
}

```

สุดท้ายยกส่วนของการจ่ายเงินพนักงานมาอยู่ที่ `EmployeePayment` 

```go
// EmployeePayment calculates and pays the employee based on their working hours and salary.
type EmployeePayment struct { ... }

func (pmt EmployeePayment) CalculatePay(emp Employee) Monay {
        return emp.SalaryPerHour() * Money(emp.TotalWorkingHours())
}
```

พอแก้ไขตามหลัก SRP แล้วจะเห็นได้ว่าเมื่อมีการแก้ไขฟังก์ชันใด ๆ ก็จะไม่กระทบกันแล้ว และยังสามารถเพิ่มประเภทของพนักงานเข้าไปได้โดยที่ไม่จำเป็นต้องแก้ไขฟังก์ชัน `CalculatePay` อีกด้วย

ท้ายนี้ขอยกคำพูดของ Uncle Bob ที่กล่าวไว้ว่า

> Gather together the things that change for the same reasons. Separate those things that change for different reasons.

---

#go #solid
