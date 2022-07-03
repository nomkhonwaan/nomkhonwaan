package main

import "testing"

// Game contains a one bowling game data such as frames, score, etc.
type Game struct {
}

// roll records number of knocked down pins.
func (g Game) roll(pins int) {
}

// score calculates and returns a total score of the game.
func (g Game) score() int {
	return 0
}

func TestGame(t *testing.T) {
	// Given
	tests := map[string]struct {
		frames   [][]int // contains slice of the knocked down pins number
		expected int
	}{
		"When no roll, should return 0": {
			expected: 0,
		},
		"When knockout 1 pin per frame, should return 10": {
			frames:   [][]int{{1}, {1}, {1}, {1}, {1}, {1}, {1}, {1}, {1}, {1}},
			expected: 10,
		},
	}

	// When
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			g := Game{}
			// play the game n frames
			for _, frame := range test.frames {
				// roll a ball and knock down the pins
				for _, pins := range frame {
					g.roll(pins)
				}
			}

			score := g.score()
			if score != test.expected {
				t.Errorf("expected %d but got: %d", test.expected, score)
			}
		})
	}
}
