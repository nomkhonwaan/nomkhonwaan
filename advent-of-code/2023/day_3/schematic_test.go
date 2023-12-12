package main

import (
	"reflect"
	"testing"
)

// func TestNumber_AdjacentCoordinators(t *testing.T) {
// 	// Given
// 	compareSlices := func(t *testing.T, got, want []Coordinator) {
// 		t.Helper()
// 		if !reflect.DeepEqual(got, want) {
// 			t.Errorf("expected %v but got %v", got, want)
// 		}
// 	}

// 	tests := map[string]struct {
// 		number   Number
// 		width    int
// 		height   int
// 		expected []Coordinator
// 	}{
// 		"With center position": {
// 			number: Number{
// 				Value: 1,
// 				Coordinators: []Coordinator{
// 					{X: 1, Y: 1},
// 				},
// 			},
// 			width:  3,
// 			height: 3,
// 			expected: []Coordinator{
// 				{X: 0, Y: 1}, {X: 0, Y: 0}, {X: 0, Y: 2},
// 				{X: 2, Y: 1}, {X: 2, Y: 0}, {X: 2, Y: 2},
// 				{X: 1, Y: 0}, {X: 1, Y: 2},
// 			},
// 		},
// 	}

// 	for testName, test := range tests {
// 		t.Run(testName, func(t *testing.T) {
// 			got := test.number.AdjacentCoordinators(test.width, test.height)
// 			compareSlices(t, got, test.expected)
// 		})
// 	}
// }

// func TestParseSymbols(t *testing.T) {
// 	// Given
// 	tests := map[string]struct {
// 		row      string
// 		expected []Symbol
// 	}{
// 		"617*......": {
// 			row: "617*......",
// 			expected: []Symbol{
// 				{
// 					Value:       "*",
// 					Coordinator: Coordinator{X: 3, Y: 0},
// 				},
// 			},
// 		},
// 	}

// 	// When
// 	for name, test := range tests {
// 		t.Run(name, func(t *testing.T) {
// 			result := ParseSymbols(0, test.row)
// 			if !reflect.DeepEqual(test.expected, result) {
// 				log.Fatalf("expected %v but got %v", test.expected, result)
// 			}
// 		})
// 	}
// }

// func TestContains(t *testing.T) {
// 	// Given
// 	coordinators := []Coordinator{
// 		{X: 0, Y: 1}, {X: 0, Y: 0}, {X: 0, Y: 2},
// 		{X: 2, Y: 1}, {X: 2, Y: 0}, {X: 2, Y: 2},
// 		{X: 1, Y: 0}, {X: 1, Y: 2},
// 	}
// 	coordinator := Coordinator{X: 0, Y: 1}

// 	// When
// 	result := IsAdjacentToSymbol(coordinators, coordinator)

// 	// Then
// 	if !result {
// 		t.Fatal("expected true but got false")
// 	}
// }

// func TestParseSchematic(t *testing.T) {
// 	// Given
// 	data := `......850...503......................*......*....411....*....674...............*...........532.124.......*.471..380.........................
// ......*........*...390..641......99...997..688..............-......403...6.....156.....................320................*721..............
// `

// 	// When
// 	result := ParseSchematic(strings.NewReader(data))

// 	// Then
// 	if !reflect.DeepEqual(Number{
// 		Value: 850,
// 		Coordinators: []Coordinator{
// 			{X: 6, Y: 0},
// 			{X: 7, Y: 0},
// 			{X: 8, Y: 0},
// 		},
// 	}, result.Numbers[0]) {
// 		t.Fatalf("not matched 850 but got %v", result)
// 	}

// 	if !reflect.DeepEqual(Number{
// 		Value: 390,
// 		Coordinators: []Coordinator{
// 			{X: 19, Y: 0},
// 			{X: 20, Y: 0},
// 			{X: 21, Y: 0},
// 		},
// 	}, result.Numbers[9]) {
// 		t.Fatalf("not matched 850 but got %v", result)
// 	}

// }

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
