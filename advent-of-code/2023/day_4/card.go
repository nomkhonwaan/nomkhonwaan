package main

import (
	"regexp"
	"strings"
)

var (
	re = regexp.MustCompile(`^Card\s+(\d+):(.+)$`)
)

// The Card contains both winning and on my hand numbers.
type Card struct {
	ID string

	WinningNumbers Numbers
	InMyHand       Numbers
}

// A Number in the card.
type Number string

type Numbers []Number

func (numbers Numbers) Contains(number Number) bool {
	for _, _number := range numbers {
		if _number == number {
			return true
		}
	}
	return false
}

func ParseCard(s string) Card {
	matches := re.FindStringSubmatch(s)
	numbers := strings.Split(matches[2], "|")
	winningNumbers := strings.TrimSpace(numbers[0])
	inMyHand := strings.TrimSpace(numbers[1])

	return Card{
		ID:             matches[1],
		WinningNumbers: ParseNumbers(winningNumbers),
		InMyHand:       ParseNumbers(inMyHand),
	}
}

func ParseNumbers(numbers string) Numbers {
	result := make(Numbers, 0)

	_numbers := strings.Split(numbers, " ")
	for _, _number := range _numbers {
		// Ignore an empty space due to a single number padding with space
		if _number == "" {
			continue
		}
		result = append(result, Number(strings.TrimSpace(_number)))
	}

	return result
}
