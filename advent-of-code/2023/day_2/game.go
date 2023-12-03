package main

import (
	"errors"
	"regexp"
	"strconv"
	"strings"
)

var (
	reGame = regexp.MustCompile(`^Game (\d+):(.+)$`)
	reCube = regexp.MustCompile(`^(\d+)\s(red|blue|green)$`)
)

// A Game object.
type Game struct {
	ID      int
	Subsets []Subset
}

// ExceedMaxCubes checks against its subsets are exceeded the maximum number of each cube or not.
func (g Game) ExceedMaxCubes(red, blue, green int) bool {
	for _, _subset := range g.Subsets {
		if _subset.Red > red {
			return true
		}
		if _subset.Blue > blue {
			return true
		}
		if _subset.Green > green {
			return true
		}
	}
	return false
}

// FewestNumberOfCubesOfEachColor returns the fewest number of each cubes respectively.
func (g Game) FewestNumberOfCubesOfEachColor() []int {
	red, blue, green := make([]int, 0), make([]int, 0), make([]int, 0)

	for _, _subset := range g.Subsets {
		if _subset.Red > 0 {
			red = append(red, _subset.Red)
		}
		if _subset.Blue > 0 {
			blue = append(blue, _subset.Blue)
		}
		if _subset.Green > 0 {
			green = append(green, _subset.Green)
		}
	}

	return []int{
		max(red...),
		max(blue...),
		max(green...),
	}
}

// Power returns a power of the game
// calculate by a fewest number of ecah cube multiplied together.
func (g Game) Power() int {
	n := g.flatten()
	return max(n[0]...) * max(n[1]...) * max(n[2]...)
}

func (g Game) flatten() [][]int {
	l := len(g.Subsets)
	red := make([]int, l)
	blue := make([]int, l)
	green := make([]int, l)

	for i, _subset := range g.Subsets {
		red[i] = _subset.Red
		blue[i] = _subset.Blue
		green[i] = _subset.Green
	}

	return [][]int{red, blue, green}
}

func max(numbers ...int) int {
	n := 0
	for _, _number := range numbers {
		if _number > n {
			n = _number
		}
	}
	return n
}

// A Subset contains each game information.
type Subset struct {
	// Set of cubes shows in each subset game.
	Red, Blue, Green int
}

// Convert each game information into an object.
func parseGame(record string) (Game, error) {
	matches := reGame.FindStringSubmatch(record)
	if len(matches) < 3 {
		return Game{}, errors.New("invalid game record")
	}

	id, err := strconv.Atoi(matches[1])
	if err != nil {
		return Game{}, err
	}

	subsets, err := parseSubsets(matches[2])
	if err != nil {
		return Game{}, err
	}

	return Game{
		ID:      id,
		Subsets: subsets,
	}, nil
}

// Convert the remaining game record into list of subsets.
func parseSubsets(record string) ([]Subset, error) {
	subsets := make([]Subset, 0)

	_subsets := strings.Split(record, ";")
	for _, _subset := range _subsets {
		// Remove extra spaces from the subset
		_subset = strings.TrimSpace(_subset)

		subset, err := parseSubset(_subset)
		if err != nil {
			return nil, err
		}

		subsets = append(subsets, subset)
	}

	return subsets, nil
}

// Convert each subset's cube information to object.
func parseSubset(record string) (subset Subset, err error) {
	_cubes := strings.Split(record, ",")
	for _, _cube := range _cubes {
		// Remove extra spaces from the cube
		_cube = strings.TrimSpace(_cube)

		matches := reCube.FindStringSubmatch(_cube)
		if len(matches) < 3 {
			return Subset{}, errors.New("invalid subset")
		}

		switch matches[2] {
		case "red":
			subset.Red, err = strconv.Atoi(matches[1])
			if err != nil {
				return
			}
		case "blue":
			subset.Blue, err = strconv.Atoi(matches[1])
			if err != nil {
				return
			}
		case "green":
			subset.Green, err = strconv.Atoi(matches[1])
			if err != nil {
				return
			}
		}
	}

	return
}
