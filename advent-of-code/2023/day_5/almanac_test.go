package main

import (
	"reflect"
	"testing"
)

func TestParseInitialSeeds(t *testing.T) {
	// Given
	expected := []uint64{79, 14, 55, 13}

	// When
	result := parseInitialSeeds("seeds: 79 14 55 13")

	// Then
	if !reflect.DeepEqual(expected, result) {
		t.Fatalf("expected %v but got %v", expected, result)
	}
}

func TestParseRangeInitialSeeds(t *testing.T) {
	// Given
	expected := []uint64{1, 2, 56, 57, 58}

	// When
	result := parseRangeInitialSeeds("seeds: 1 2 56 3")

	// Then
	if !reflect.DeepEqual(expected, result) {
		t.Fatalf("expected %v but got %v", expected, result)
	}
}
