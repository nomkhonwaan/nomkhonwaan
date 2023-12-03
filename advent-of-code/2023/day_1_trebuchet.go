package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
	"strconv"
)

var (
	re = regexp.MustCompile(`[a-zA-Z]`)
)

func main() {
	f, err := os.Open("input.txt")
	checkErr(err)
	defer func() { _ = f.Close() }()

	b := bufio.NewScanner(f)

	var firstPartAnswer, secondPartAnswer int

	for b.Scan() {
		value := b.Text()

		firstPartAnswer += calFirstPartAnswer(value)
		secondPartAnswer += calSecondPartAnswer(value)
	}

	log.Printf("The first part answer is: %d\n", firstPartAnswer)
	log.Printf("The second part answer is: %d", secondPartAnswer)
}

func checkErr(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func calFirstPartAnswer(value string) int {
	_value := getOnlyFirstAndLastDigits(removeNonDigitCharacters(value))

	n, _ := strconv.Atoi(_value)
	return n
}

func calSecondPartAnswer(value string) int {
	_value := getOnlyFirstAndLastDigits(convCalibrationValueToDigit(value))

	n, _ := strconv.Atoi(_value)
	return n
}

func removeNonDigitCharacters(value string) string {
	return re.ReplaceAllString(value, "")
}

func getOnlyFirstAndLastDigits(digits string) string {
	if len(digits) < 1 {
		return ""
	}

	// Some value are only 1 digit,
	// so this is to make sure all of them should have at least 2 digits.
	_digits := digits + digits
	return _digits[0:1] + _digits[len(_digits)-1:]
}

func convCalibrationValueToDigit(value string) string {
	m := map[string]string{
		"one": "1", "two": "2", "three": "3",
		"four": "4", "five": "5", "six": "6",
		"seven": "7", "eight": "8", "nine": "9",
	}

	// Do not calculate the value since it is too short
	if len(value) < 1 {
		return value
	}

	for i := 0; i < len(value); i++ {
		// The character is a digit,
		// so keep the digit and try parsing the remaining value.
		if value[i] >= '1' && value[i] <= '9' {
			return string(value[i]) + convCalibrationValueToDigit(value[i+1:])
		}

		// The maximum number letter is five characters,
		// so wether reaching the maximum or end of string should be return.
		for j := i; j-i <= 5 && j <= len(value); j++ {
			// The first word is matching with the number letter,
			// but the other might also match as well.
			if v, ok := m[value[i:j]]; ok {
				// So, keep the first matched number letter and try parising
				// the remaining value as well.
				return v + convCalibrationValueToDigit(value[j-1:])
			}
		}
	}

	// No matching any patterns, try parsing the next letter.
	return convCalibrationValueToDigit(value[1:])
}
