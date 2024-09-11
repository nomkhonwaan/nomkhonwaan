package main

import (
	"reflect"
	"testing"
)

func TestParseMap(t *testing.T) {
	// Given
	expected := Map{
		records: []Record{
			{src: 98, dst: 50, length: 2},
			{src: 7, dst: 57, length: 4},
		},
	}

	// When
	result := parseMap([]string{"50 98 2", "57 7 4"})

	// Then
	if !reflect.DeepEqual(expected, result) {
		t.Fatalf("expected %v but got %v", expected, result)
	}
}

func TestMap_GetDestination(t *testing.T) {
	// Given
	tests := []struct {
		m        Map
		target   uint64
		expected uint64
	}{
		{
			m: Map{
				records: []Record{
					{src: 1, dst: 10, length: 2},
				},
			},
			target:   3,
			expected: 3,
		},
		{
			m: Map{
				records: []Record{
					{src: 1, dst: 10, length: 2},
					{src: 50, dst: 30, length: 7},
				},
			},
			target:   55,
			expected: 35,
		},
	}

	// When
	for _, test := range tests {
		result := test.m.GetDestination(test.target)
		if !reflect.DeepEqual(test.expected, result) {
			t.Fatalf("expected %v but got %v", test.expected, result)
		}
	}
}

func TestMap_GetSource(t *testing.T) {
	// Given
	tests := []struct {
		m        Map
		target   uint64
		expected uint64
	}{
		{
			m: Map{
				records: []Record{
					{src: 1, dst: 10, length: 2},
				},
			},
			target:   3,
			expected: 3,
		},
		{
			m: Map{
				records: []Record{
					{src:1, dst:10, length: 2},
					{src: 50, dst: 30, length: 7},
				},
			},
			target: 36,
			expected: 56,
		},
	}

	// When
	for _, test := range tests {
		result := test.m.GetSource(test.target)
		if !reflect.DeepEqual(test.expected, result) {
			t.Fatalf("expected %v but got %v", test.expected, result)
		}
	}
}
