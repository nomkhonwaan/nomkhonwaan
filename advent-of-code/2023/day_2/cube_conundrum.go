package main

import (
	"bufio"
	"log"
	"os"
)

const (
	maxRed   = 12
	maxGreen = 13
	maxBlue  = 14
)

func main() {
	f, err := os.Open("input.txt")
	checkErr(err)
	defer func() { _ = f.Close() }()

	b := bufio.NewScanner(f)

	var firstPartAnswer, secondPartAnswer int

	for b.Scan() {
		g, err := parseGame(b.Text())
		if err != nil {
			log.Fatal(err)
		}

		firstPartAnswer += calFirstPartAnswer(g)
		secondPartAnswer += calSecondPartAnswer(g)
	}

	log.Printf("The first part answer is: %d\n", firstPartAnswer)
	log.Printf("The second part answer is: %d", secondPartAnswer)
}

func checkErr(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func calFirstPartAnswer(g Game) int {
	if !g.ExceedMaxCubes(maxRed, maxBlue, maxGreen) {
		return g.ID
	}

	return 0
}

func calSecondPartAnswer(g Game) int {
	return g.Power()
}
