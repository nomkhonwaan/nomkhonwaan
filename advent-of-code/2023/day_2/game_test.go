package main

import (
	"log"
	"reflect"
	"testing"
)

func TestGame_ExceedMaxCubes(t *testing.T) {
	// Given
	g := Game{Subsets: []Subset{{Red: 6, Blue: 1, Green: 3}, {Blue: 2, Red: 1, Green: 2}}}

	// When
	result := g.ExceedMaxCubes(12, 13, 14)

	// Then
	if result {
		t.Fatalf("expected false but got %v", result)
	}
}

func TestGame_Power(t *testing.T) {
	// Given
	g := Game{Subsets: []Subset{{Blue: 3, Red: 4}, {Red: 1, Green: 2, Blue: 6}, {Green: 2}}}

	// When
	result := g.Power()

	// Then
	if result != 48 {
		t.Fatalf("expected %d but got %d", 48, result)
	}
}

func TestMin(t *testing.T) {
	// Given
	tests := map[string]struct {
		numbers  []int
		expected int
	}{
		"0, 1, 2 ,3, 4": {
			numbers:  []int{0, 1, 2, 3, 4},
			expected: 1,
		},
	}

	//
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			result := max(test.numbers...)
			if result != test.expected {
				log.Fatalf("expected %d but got %d", test.expected, result)
			}
		})
	}
}

func TestParseGame(t *testing.T) {
	// Given
	record := "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"

	expected := Game{
		ID: 5,
		Subsets: []Subset{
			{
				Red:   6,
				Blue:  1,
				Green: 3,
			},
			{
				Blue:  2,
				Red:   1,
				Green: 2,
			},
		},
	}

	// When
	result, err := parseGame(record)

	// Then
	if err != nil {
		t.Fatalf("expected an error to be nil but got %s", err)
	}

	if !reflect.DeepEqual(expected, result) {
		t.Fatalf("expected %v but got %v", expected, result)
	}
}

func TestParseSubsets(t *testing.T) {
	// Given
	tests := map[string]struct {
		record   string
		expected []Subset
	}{
		"Game 1": {
			record: " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
			expected: []Subset{
				{
					Blue: 3,
					Red:  4,
				},
				{
					Red:   1,
					Green: 2,
					Blue:  6,
				},
				{
					Green: 2,
				},
			},
		},
		"Game 4": {
			record: " 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
			expected: []Subset{
				{
					Green: 1,
					Red:   3,
					Blue:  6,
				},
				{
					Green: 3,
					Red:   6,
				},
				{
					Green: 3,
					Blue:  15,
					Red:   14,
				},
			},
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			result, err := parseSubsets(test.record)
			if err != nil {
				t.Fatalf("expected an error to be nil but got %s", err)
			}

			if !reflect.DeepEqual(test.expected, result) {
				t.Fatalf("expected %v but got %v", test.expected, result)
			}
		})
	}
}
