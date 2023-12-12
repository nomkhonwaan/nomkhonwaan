package main

import (
	"bufio"
	"log"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	checkErr(err)
	defer func() { _ = f.Close() }()

	b := bufio.NewScanner(f)

	cards := make([]Card, 0)
	for b.Scan() {
		cards = append(cards, ParseCard(b.Text()))
	}

	log.Printf("The first part answer is: %d\n", calFirstPartAnswer(cards))
	log.Printf("The second part answer is: %d", calSecondPartAnswer(cards))
}

func checkErr(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func calFirstPartAnswer(cards []Card) int {
	var total int

	for _, c := range cards {
		var sum int

		for _, _number := range c.InMyHand {
			if c.WinningNumbers.Contains(_number) {
				if sum < 1 {
					sum = 1
				} else {
					sum += sum
				}
			}
		}

		total += sum
	}

	return total
}

func calSecondPartAnswer(cards []Card) int {
	var total int
	copies := make([]int, len(cards))

	for i, c := range cards {
		copies[i] += 1

		var matching int
		for _, _number := range c.InMyHand {
			if c.WinningNumbers.Contains(_number) {
				matching += 1
				copies[i+matching] += copies[i]
			}
		}

		total += copies[i]
	}

	return total
}
