package main

import (
	"net/http"
	"time"
)

// Post represents a single post entity in the `posts` table.
type Post struct {
	ID        int
	Title     string
	Body      string
	CreatedAt *time.Time
	UpdatedAt *time.Time
}

func main() {
	// Display all posts in JSON format
	http.HandleFunc("/posts", findAll)

	// Serve HTTP server on http://localhost:8080
	http.ListenAndServe(":8080", nil)
}

// findAll shows all posts in the database.
func findAll(w http.ResponseWriter, r *http.Request) {
	
}
