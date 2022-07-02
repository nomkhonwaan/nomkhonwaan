package main

import (
	"regexp"
	"strconv"
	"strings"
	"testing"
)

func Add(numbers string) int {
	if numbers == "" {
		return 0
	}

	n := toNumbers(parseString(numbers))
	var sum int
	for _, i := range n {
		sum += i
	}

	return sum
}

// convert numbers string into slice of integer.
func toNumbers(delimiter, numbers string) []int {
	numbers = strings.ReplaceAll(numbers, "\n", delimiter)
	n := strings.Split(numbers, delimiter)
	m := make([]int, 0)

	for _, i := range n {
		j, err := strconv.Atoi(i)
		if err != nil {
			panic(err)
		}
		m = append(m, j)
	}

	return m
}

// parse numbers string and return both delimiter (if exist or default comma) and numbers string.
func parseString(numbers string) (string, string) {
	re := regexp.MustCompile(`^//(.+)\n(.+)`)
	matches := re.FindStringSubmatch(numbers)

	// This means the given numbers string matches with the pattern,
	// then set the delimiter with the given string.
	if len(matches) == 3 {
		return matches[1], matches[2]
	}

	// There is no matching pattern, so just return the given numbers string
	// with the default delimiter.
	return ",", numbers
}

func TestAdd(t *testing.T) {
	// Given
	tests := map[string]struct {
		input     string
		expected  int
		willPanic bool
	}{
		"Should return 0": {
			input:    "",
			expected: 0,
		},
		"Should return 1": {
			input:    "1",
			expected: 1,
		},
		"Should return 3": {
			input:    "1,2",
			expected: 3,
		},
		"Should return -1": {
			input:    "-2,1",
			expected: -1,
		},
		"Should return 90": {
			input:    "89,1",
			expected: 90,
		},
		"Should return 6": {
			input:    "1\n2,3",
			expected: 6,
		},
		"Should panic": {
			input:     "1\n,",
			willPanic: true,
		},
		"With semicolon, should return 3": {
			input:    "//;\n1;2",
			expected: 3,
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			if test.willPanic {
				defer func() {
					if r := recover(); r == nil {
						t.Error("expected panic")
					}
				}()
			}

			result := Add(test.input)
			if result != test.expected {
				t.Errorf("expected %d but got: %d", test.expected, result)
			}
		})
	}
}
