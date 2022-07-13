package main

import (
        "database/sql"
        "encoding/json"
        "fmt"
        "net/http"
        "net/http/httptest"
        "testing"
        "time"

        _ "github.com/go-sql-driver/mysql"
)

// Repository provides `posts` handling function
type Repository interface {
        FindAll() ([]Post, error)
}

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

// Post represents a single post entity in the `posts` table.
type Post struct {
        ID        int
        Title     string
        Body      string
        CreatedAt *time.Time
        UpdatedAt *time.Time
}

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