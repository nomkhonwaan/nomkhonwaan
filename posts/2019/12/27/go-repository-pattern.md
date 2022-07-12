---
title: Repository Pattern ใน Go
publish_date: 2019-12-27
---

ภาษากลุ่มที่เป็น Object-oriented Programming (OOP) จะพบรูปแบบการใช้งาน Repository Pattern เป็นปกติ ข้อดีของรูปแบบนี้คือการแยกส่วนของประมวลผล (Business Logic Layer: BLL) ออกจากส่วนของการติดต่อกับข้อมูล (Data Access Lager: DAL) ซึ่งสอดคล้องกับหลักการเขียนโปรแกรมที่ดีคือ [Low Coupling, High Cohesion](https://en.wikipedia.org/wiki/Loose_coupling)

มาลองดูกันว่าหลักการนี้สามารถนำมาใช้กับ Go อย่างไรได้บ้าง

---

สมมติว่าต้องการพัฒนาเว็บไซต์โดยมีข้อมูลบทความจัดเก็บอยู่ใน RDBMS แบบนี้ (ในที่นี้คือ MariaDB)

```sql
DROP TABLE IF EXISTS posts;

CREATE TABLE posts (
  id INT(6) UNSIGNED AUTO_INCREMENT PRIMARY KEY,
  title VARCHAR(255),
  body TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME ON UPDATE CURRENT_TIMESTAMP
);

INSERT INTO posts (id, title, body) VALUES (1, 'sunt aut facere repellat provident occaecati excepturi optio reprehenderit', 'quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto');
INSERT INTO posts (id, title, body) VALUES (2, 'qui est esse', 'est rerum tempore vitae\nsequi sint nihil reprehenderit dolor beatae ea dolores neque\nfugiat blanditiis voluptate porro vel nihil molestiae ut reiciendis\nqui aperiam non debitis possimus qui neque nisi nulla');
INSERT INTO posts (id, title, body) VALUES (3, 'ea molestias quasi exercitationem repellat qui ipsa sit aut', 'et iusto sed quo iure\nvoluptatem occaecati omnis eligendi aut ad\nvoluptatem doloribus vel accusantium quis pariatur\nmolestiae porro eius odio et labore et velit aut');
INSERT INTO posts (id, title, body) VALUES (4, 'eum et est occaecati', 'ullam et saepe reiciendis voluptatem adipisci\nsit amet autem assumenda provident rerum culpa\nquis hic commodi nesciunt rem tenetur doloremque ipsam iure\nquis sunt voluptatem rerum illo velit');
INSERT INTO posts (id, title, body) VALUES (5, 'nesciunt quas odio', 'repudiandae veniam quaerat sunt sed\nalias aut fugiat sit autem sed est\nvoluptatem omnis possimus esse voluptatibus quis\nest aut tenetur dolor neque');
```

ที่ฝั่งของ Go สร้าง `struct` แบบนี้

```go
// Post represents a single post entity in the `posts` table.
type Post struct {
        ID        int
        Title     string
        Body      string
        CreatedAt *time.Time
        UpdatedAt *time.Time
}
```

ทีนี้ถ้าต้องการแสดงผลข้อมูลโพสต์ทั้งหมดในรูปแบบ JSON กรณีที่ไม่ใช้งาน Repository กระบวนการคิวรี่ข้อมูลก็จะเขียนไว้ที่ฟังก์ชันนั้น ๆ แทนแบบนี้

```go

```

---
#go #repository-pattern #design-pattern