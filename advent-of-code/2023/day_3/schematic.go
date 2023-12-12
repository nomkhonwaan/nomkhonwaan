package main

import (
	"bufio"
	"io"
	"strconv"
)

// Schematic consists of a visual representation of the engine.
type Schematic struct {
	Width, Height int

	Symbols []Symbol
	Numbers []Number
}

// A Symbol of an engine's schematic.
type Symbol struct {
	Value       string
	Coordinator Coordinator
}

// A Number of an engine's schematics.
type Number struct {
	Value        int
	Coordinators []Coordinator
}

// AdjacentCoordinators returns a list of coordinators are adjacent to the number.
func (n Number) AdjacentCoordinators(width, height int) []Coordinator {
	var coordinators []Coordinator

	// Every coordinator is adjacent to each other by default
	copy(coordinators, n.Coordinators)

	directions := []Coordinator{
		{X: -1, Y: -1}, // top-left
		{X: -1, Y: 0},  // left
		{X: -1, Y: 1},  // bottom-left
		{X: 0, Y: -1},  // top
		{X: 0, Y: 1},   // bottom
		{X: 1, Y: -1},  // top-right
		{X: 1, Y: 0},   // right
		{X: 1, Y: 1},   // bottom-right
	}

	for _, _coordinator := range n.Coordinators {
		for _, _direction := range directions {
			x := _coordinator.X + _direction.X
			y := _coordinator.Y + _direction.Y

			if x >= 0 && x < width && y >= 0 && y <= height {
				coordinators = append(coordinators, NewCoordinator(x, y))
			}
		}
	}

	return coordinators
}

func IsAdjacentToSymbol(coordinators []Coordinator, coordinator Coordinator) bool {
	for _, _coordinator := range coordinators {
		if _coordinator.X == coordinator.X &&
			_coordinator.Y == coordinator.Y {
			return true
		}
	}
	return false
}

// Coordinator is an X, Y position in the 2-dimensions array schematic.
type Coordinator struct {
	X, Y int
}

func NewCoordinator(x, y int) Coordinator {
	return Coordinator{X: x, Y: y}
}

// ParseSchematic parses an io.Reader to the 2-dimension array.
func ParseSchematic(r io.Reader) Schematic {
	schematic := Schematic{Numbers: make([]Number, 0)}

	var y int

	b := bufio.NewScanner(r)
	for b.Scan() {
		row := b.Text()
		schematic.Width = len(row)

		schematic.Symbols = append(schematic.Symbols, ParseSymbols(y, row)...)
		schematic.Numbers = append(schematic.Numbers, ParseNumbers(0, y, row)...)

		y += 1
	}

	schematic.Height = y
	return schematic
}

func ParseSymbols(y int, row string) []Symbol {
	symbols := make([]Symbol, 0)

	for x, c := range row {
		if !IsDigit(c) && c != '.' {
			symbols = append(symbols, Symbol{
				Value:       string(c),
				Coordinator: NewCoordinator(x, y),
			})
		}
	}

	return symbols
}

func ParseNumbers(x, y int, row string) []Number {
	var (
		value        = make([]rune, 0)
		coordinators = make([]Coordinator, 0)
	)

	for i, c := range row {
		if IsDigit(c) {
			value = append(value, c)
			coordinators = append(coordinators, NewCoordinator(x, y))
		} else {
			if len(value) > 0 {
				number := Number{Coordinators: coordinators}
				number.Value, _ = strconv.Atoi(string(value))

				return append([]Number{number}, ParseNumbers(x, y, row[i:])...)
			}
		}

		x += 1
	}

	// For the latest number in the line.
	if len(value) > 0 {
		number := Number{Coordinators: coordinators}
		number.Value, _ = strconv.Atoi(string(value))

		return []Number{number}
	}

	return nil
}

func IsDigit(c rune) bool {
	return c >= '0' && c <= '9'
}
