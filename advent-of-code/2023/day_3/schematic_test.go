package main

import (
	"log"
	"reflect"
	"testing"
)

func TestParseSymbols(t *testing.T) {
	// Given
	tests := map[string]struct {
		row      string
		expected []Symbol
	}{
		"617*......": {
			row: "617*......",
			expected: []Symbol{
				{
					Value:       "*",
					Coordinator: Coordinator{X: 3, Y: 0},
				},
			},
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			result := ParseSymbols(0, test.row)
			if !reflect.DeepEqual(test.expected, result) {
				log.Fatalf("expected %v but got %v", test.expected, result)
			}
		})
	}
}

func TestContains(t *testing.T) {
	// Given
	coordinators := []Coordinator{
		{X: 0, Y: 1}, {X: 0, Y: 0}, {X: 0, Y: 2},
		{X: 2, Y: 1}, {X: 2, Y: 0}, {X: 2, Y: 2},
		{X: 1, Y: 0}, {X: 1, Y: 2},
	}
	coordinator := Coordinator{X: 0, Y: 1}

	// When
	result := IsAdjacentToSymbol(coordinators, coordinator)

	// Then
	if !result {
		t.Fatal("expected true but got false")
	}
}

func TestParseNumbers(t *testing.T) {
	// Given
	tests := map[string]struct {
		row      string
		expected []Number
	}{
		"467..114": {
			row: "467..114",
			expected: []Number{
				{
					Value: 467,
					Coordinators: []Coordinator{
						{X: 0, Y: 0},
						{X: 1, Y: 0},
						{X: 2, Y: 0},
					},
				},
				{
					Value: 114,
					Coordinators: []Coordinator{
						{X: 5, Y: 0},
						{X: 6, Y: 0},
						{X: 7, Y: 0},
					},
				},
			},
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			result := ParseNumbers(0, 0, test.row)
			if !reflect.DeepEqual(test.expected, result) {
				t.Fatalf("expected %v but got %v", test.expected, result)
			}
		})
	}
}
