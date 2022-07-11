---
title: Repository Pattern ใน Go
publish_date: 2019-12-27
---

ภาษากลุ่มที่เป็น Object-oriented Programming (OOP) จะพบรูปแบบการใช้งาน Repository Pattern เป็นปกติ ข้อดีของรูปแบบนี้คือการแยกส่วนของประมวลผล (Business Logic Layer: BLL) ออกจากส่วนของการติดต่อกับข้อมูล (Data Access Lager: DAL) ซึ่งสอดคล้องกับหลักการเขียนโปรแกรมที่ดีคือ [Low Coupling, High Cohesion](https://en.wikipedia.org/wiki/Loose_coupling)

มาลองดูกันว่าหลักการนี้สามารถนำมาใช้กับ Go อย่างไรได้บ้าง

---



---
#go #repository-pattern #design-pattern