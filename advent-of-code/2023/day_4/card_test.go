package main

import (
	"reflect"
	"testing"
)

func TestParseCard(t *testing.T) {
	// Given
	s := "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"
	expected := Card{
		ID:             "3",
		WinningNumbers: []Number{"1", "21", "53", "59", "44"},
		InMyHand:       []Number{"69", "82", "63", "72", "16", "21", "14", "1"},
	}

	// When
	result := ParseCard(s)

	// Then
	if !reflect.DeepEqual(expected, result) {
		t.Fatalf("expected %v but got %v", expected, result)
	}
}
