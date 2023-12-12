package main

import (
	"log"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	checkErr(err)
	defer func() { _ = f.Close() }()

	var firstPartAnswer, secondPartAnswer int

	schematic := ParseSchematic(f)

	firstPartAnswer = calFirstPartAnswer(schematic)
	secondPartAnswer = calSecondPartAnswer(schematic)

	log.Printf("The first part answer is: %d\n", firstPartAnswer)
	log.Printf("The second part answer is: %d", secondPartAnswer)
}

func checkErr(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func calFirstPartAnswer(schematic Schematic) int {
	var sum int

	for _, _number := range schematic.Numbers {
		for _, _symbol := range schematic.Symbols {
			if IsAdjacentToSymbol(
				_number.AdjacentCoordinators(schematic.Width, schematic.Width),
				_symbol.Coordinator,
			) {
				sum += _number.Value
			}
		}
	}

	return sum
}

func calSecondPartAnswer(schematic Schematic) int {
	var sum int

	for _, _symbol := range schematic.Symbols {
		if _symbol.Value != "*" {
			continue
		}

		numbers := make([]Number, 0)
		for _, _number := range schematic.Numbers {
			if IsAdjacentToSymbol(
				_number.AdjacentCoordinators(schematic.Width, schematic.Height),
				_symbol.Coordinator,
			) {
				numbers = append(numbers, _number)
			}
		}

		if len(numbers) != 2 {
			continue
		}

		sum += numbers[0].Value * numbers[1].Value
	}

	return sum
}
