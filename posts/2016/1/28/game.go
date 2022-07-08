package main

import (
	"testing"
)

// Game contains a one bowling game data such as frames, score, etc.
type Game struct {
	times  int
	frames []Frame
}

// roll records number of knocked down pins.
func (g *Game) roll(pins int) {
	g.times++
	// this is the first attempt,
	// create a new frame instance and
	// put the number of knocked down pin in it.
	if g.times%2 != 0 {
		// strike!
		// let's skip the second roll of this frame
		if pins == 10 {
			g.times++
		}
		// either spare or strike at the last frame,
		// then the knocked down pins will assign to the special score
		// of the last frame.
		if float64(g.times)/2 > 10 {
			g.frames[9].lastFrameKnockedPins += pins
			return
		}

		g.frames = append(g.frames, Frame{firstKnockedPins: pins})
		return
	}
	// get the latest frame from the list of frames
	// and update its roll.
	g.frames[len(g.frames)-1].lastKnockedPins = pins
}

// score calculates and returns a total score of the game.
func (g Game) score() int {
	var totalScore int

	for i, f := range g.frames {
		totalScore += f.firstKnockedPins + f.lastKnockedPins + f.lastFrameKnockedPins // include the last frame special score

		// when the current frame able to spare all pins,
		// get the first knocked down pin in the
		// next frame's rolls plus with the total score.
		if f.isSpared() && i+1 < len(g.frames) {
			totalScore += g.frames[i+1].firstKnockedPins
		}

		// when the current frame able to strike all pins,
		// get the first and second knocked down pins in the
		// next frame's rolls plus with the total score.
		if f.isStriked() && i+1 < len(g.frames) {
			// in case of the next frame also strike,
			// then use the next first knocked down pins
			// of the next 2 frames instead.
			if g.frames[i+1].isStriked() && i+2 < len(g.frames) {
				totalScore += g.frames[i+1].firstKnockedPins + g.frames[i+2].firstKnockedPins
				continue
			}
			totalScore += g.frames[i+1].firstKnockedPins + g.frames[i+1].lastKnockedPins
		}
	}

	return totalScore
}

// Frame contains first and last knocked down pins
// for using in the total score calculation.
type Frame struct {
	firstKnockedPins, lastKnockedPins int
	lastFrameKnockedPins              int
}

// isSpared returns true when the first knocked down pins less than 10
// and the sum of first and last knocked down pins equal 10.
func (f Frame) isSpared() bool {
	return f.firstKnockedPins < 10 && f.firstKnockedPins+f.lastKnockedPins == 10
}

// isStrike returns true when the first knocked down pins is 10.
func (f Frame) isStriked() bool {
	return f.firstKnockedPins == 10
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
			frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
			expected: 10,
		},
		"When able to spare at the 2nd frame, should return 20": {
			frames:   [][]int{{1, 0}, {1, 9}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
			expected: 20,
		},
		"When able to strike at the 3rd frame, should return 20": {
			frames:   [][]int{{1, 0}, {1, 0}, {10}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
			expected: 20,
		},
		"When able to strike at the 2nd frame and spare at the 3rd frame, should return 39": {
			frames:   [][]int{{1, 0}, {10}, {1, 9}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
			expected: 39,
		},
		"When able to strike at the 1st and 2nd frames, should return 46": {
			frames:   [][]int{{10}, {10}, {1, 3}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}},
			expected: 46,
		},
		"When able to spare at the last frame, should return 24": {
			frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {3, 7, 5}},
			expected: 24,
		},
		"When able to strike at the last frame, should return 22": {
			frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {10, 1, 2}},
			expected: 22,
		},
		"When able to strike at the last frame and able to keep strike for the next 2 turns, should return 39": {
			frames:   [][]int{{1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {1, 0}, {10, 10, 10}},
			expected: 39,
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
