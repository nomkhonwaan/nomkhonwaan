package main

import (
	"testing"
)

func TestRemoveNonDigitCharacters(t *testing.T) {
	// Given
	tests := map[string]struct {
		value    string
		expected string
	}{
		"Should be 42": {
			value:    "4nineeightseven2",
			expected: "42",
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			result := removeNonDigitCharacters(test.value)
			if result != test.expected {
				t.Fatalf("expected %s but got %s", test.expected, result)
			}
		})
	}
}

func TestConvCalibrationValueToDigit(t *testing.T) {
	// Given
	tests := map[string]struct {
		value    string
		expected string
	}{
		"Should be 29": {
			value:    "two1nine",
			expected: "219",
		},
		"Should be 123": {
			value:    "abcone2threexyz",
			expected: "123",
		},
		"Should be 955295": {
			value:    "ninefivefive2nine5ntvscdfdsmvqgcbxxxt",
			expected: "955295",
		},
		"Should be 2": {
			value:    "2fdmhrbdssf",
			expected: "2",
		},
		"Should be 83": {
			value:    "eighthree",
			expected: "83",
		},
		"Should be 79": {
			value:    "sevenine",
			expected: "79",
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			result := convCalibrationValueToDigit(test.value)
			if result != test.expected {
				t.Fatalf("expected %s but got: %s", test.expected, result)
			}
		})
	}
}

func TestGetOnlyFirstAndLastDigits(t *testing.T) {
	// Given
	tests := map[string]struct {
		digits   string
		expected string
	}{
		"Should return 123": {
			digits:   "123",
			expected: "13",
		},
		"Should return 77": {
			digits:   "7",
			expected: "77",
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			result := getOnlyFirstAndLastDigits(test.digits)
			if result != test.expected {
				t.Fatalf("expected %s but got %s", test.expected, result)
			}
		})
	}
}
