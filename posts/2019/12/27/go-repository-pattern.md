---
title: Repository Pattern ใน Go
publish_date: 2019-12-27
tags: ['go']
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

ทีนี้ถ้าต้องการแสดงผลข้อมูลโพสต์ทั้งหมดในรูปแบบ JSON กรณีที่ไม่ใช้งาน Repository กระบวนการคิวรี่ข้อมูลก็จะเขียนไว้ที่ฟังก์ชันตรง ๆ แบบฟังก์ชัน `listAllPosts`

```go
import (
        "database/sql"
        "encoding/json"
        "fmt"
        "net/http"
        "time"

        _ "github.com/go-sql-driver/mysql"
)

func main() {
        db, err := sql.Open("mysql", fmt.Sprintf("%s:%s@tcp(%s:3306)/%s?parseTime=true", "root", "my-secret-pw", "localhost", "go_repository_pattern"))
        if err != nil {
                panic(err)
        }
        defer db.Close()

        http.HandleFunc("/posts", listAllPosts(db))
        http.ListenAndServe(":8080", nil)
}

// listAllPosts shows all posts in the database.
func listAllPosts(db *sql.DB) http.HandlerFunc {
        return func(w http.ResponseWriter, r *http.Request) {
                rows, err := db.Query(`SELECT id, title, body, created_at, updated_at FROM posts`)
                if err != nil {
                        http.Error(w, err.Error(), http.StatusInternalServerError)
                        return
                }
                defer rows.Close()

                var posts []Post
                for rows.Next() {
                        var p Post
                        err = rows.Scan(&p.ID, &p.Title, &p.Body, &p.CreatedAt, &p.UpdatedAt)
                        if err != nil {
                                http.Error(w, err.Error(), http.StatusInternalServerError)
                                return
                        }
                        posts = append(posts, p)
                }

                err = json.NewEncoder(w).Encode(posts)
                if err != nil {
                        http.Error(w, err.Error(), http.StatusInternalServerError)
                        return
                }
        }
}
```

ลองปรับมาใช้ Repository Pattern ด้วยการรับอินเตอร์เฟส `Repository` แล้วเรียกใช้ฟังก์ชัน `FindAll` แทนแบบนี้

```go
// Repository provides `posts` handling function
type Repository interface {
        FindAll() ([]Post, error)
}

// listAllPosts shows all posts in the database.
func listAllPosts(repo Repository) http.HandlerFunc {
        return func(w http.ResponseWriter, r *http.Request) {
                posts, err := repo.FindAll()
                if err != nil {
                        http.Error(w, err.Error(), http.StatusInternalServerError)
                        return
                }

                err = json.NewEncoder(w).Encode(posts)
                if err != nil {
                        http.Error(w, err.Error(), http.StatusInternalServerError)
                        return
                }
        }
}
```

จากนั้นย้ายส่วนของการคิวรี่ข้อมูลไปอยู่ที่ `MariaDBRepository` ที่อิมพลิเมนต์ `Repository` แทน

```go
// MariaDBRepository implements Repository for using with MariaDB.
type MariaDBRepository struct {
        db *sql.DB
}

func (repo MariaDBRepository) FindAll() ([]Post, error) {
        rows, err := repo.db.Query(`SELECT id, title, body, created_at, updated_at FROM posts`)
        if err != nil {
                return nil, err
        }
        defer rows.Close()

        var posts []Post
        for rows.Next() {
                var p Post
                err = rows.Scan(&p.ID, &p.Title, &p.Body, &p.CreatedAt, &p.UpdatedAt)
                if err != nil {
                        return nil, err
                }
                posts = append(posts, p)
        }

        return posts, nil
}
```

ทำการแก้ไขฟังก์ชันที่ `main` เพื่อให้ส่ง `Repository` เข้ามาแทนที่จะเป็น `*sql.DB` 

```go
func main() {
        db, err := sql.Open("mysql", fmt.Sprintf("%s:%s@tcp(%s:3306)/%s?parseTime=true", "root", "my-secret-pw", "localhost", "go_repository_pattern"))
        if err != nil {
                panic(err)
        }
        defer db.Close()

        repo := MariaDBRepository{db: db}

        http.HandleFunc("/posts", listAllPosts(repo))
        http.ListenAndServe(":8080", nil)
}
```

ในอนาคตหากโปรแกรมต้องการเปลี่ยนจาก MariaDB เป็นฐานข้อมูลชนิดอื่น การทำงานของฟังก์ชัน `listAllPosts` ก็จะยังคงเหมือนเดิม เพราะส่วนของการติดต่อกับข้อมูลได้แยกออกจากการประมวลอยู่แล้ว

---

ข้อดีอีกอย่างคือการเขียนเทสสามารถทำได้ง่ายขึ้นโดยการจำลอง `Repository` แบบนี้

```go
type mockRepository struct{}

func (repo mockRepository) FindAll() ([]Post, error) {
        return []Post{
                {
                        ID:    1,
                        Title: "title",
                        Body:  "body",
                },
        }, nil
}

func TestSuccessfulListAllPosts(t *testing.T) {
        // Given
        w := httptest.NewRecorder()
        expected := `[{"ID":1,"Title":"title","Body":"body","CreatedAt":null,"UpdatedAt":null}]` + "\n"

        // When
        listAllPosts(mockRepository{}).ServeHTTP(w, nil)

        // Then
        if w.Body.String() != expected {
                t.Error("invalid body")
        }
}
```

[Go Playground](https://go.dev/play/p/9e9JKD1Svfx)