package main

import (
	"bufio"
	"log"
	"os"
	"slices"
	"sort"
	"strconv"
	"strings"
)

var (
	seeds []uint64
	maps  []Map
)

func main() {
	f, err := os.Open("input.txt")
	checkErr(err)
	defer func() { _ = f.Close() }()

	b := bufio.NewScanner(f)

	for i := 0; b.Scan(); i++ {
		value := b.Text()

		if i > 0 {
			if isMap(value) {
				maps = append(maps, parseMap(fetch(b)))
			}
		} else {
			seeds = parseInitialSeeds(value)
		}
	}

	log.Printf("The first part answer is: %d\n", calFirstPartAnswer(seeds))
	log.Printf("The second part answer is: %d\n", calSecondPartAnswer(seeds))
}

func checkErr(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func calFirstPartAnswer(seeds []uint64) uint64 {
	result := make([]uint64, len(seeds))

	for i, seed := range seeds {
		src := seed
		for _, m := range maps {
			dst := m.GetDestination(src)

			// assign the new destination to the source
			// for using as a new target in the next map
			src = dst
		}

		// store the location value in to the result list
		result[i] = src
	}

	return lowest(result)
}

func calSecondPartAnswer(seeds []uint64) uint64 {
	// get the latest map from the list which is location map
	location := maps[len(maps)-1]

	// sort the record destination descending
	sort.Sort(location.records)

	// get the greatest one
	r := location.records[0]

	// search back from the location, humidity, tempurature, light, fertilizer, water, soil and seed
	slices.Reverse(maps)

	for i := uint64(0); i < r.dst+r.length; i++ {
		dst := i
		for _, m := range maps {
			src := m.GetSource(dst)
			dst = src
		}

		for j := 0; j < len(seeds); j += 2 {
			if seeds[j] <= dst && dst <= seeds[j]+seeds[j+1] {
				return i
			}
		}
	}

	panic("unable to find the answer")
}

func fetch(b *bufio.Scanner) []string {
	buf := make([]string, 0)
	for b.Scan() {
		line := b.Text()
		if isBlank(line) {
			return buf
		}
		buf = append(buf, line)
	}
	return buf
}

func isMap(value string) bool {
	return strings.Contains(value, "map:")
}

func isBlank(value string) bool {
	return value == ""
}

func parseInitialSeeds(s string) []uint64 {
	v := strings.Split(s, " ")
	seeds := make([]uint64, 0)
	// strip the first value "seeds:"
	for _, seed := range v[1:] {
		i, _ := strconv.ParseUint(seed, 10, 64)
		seeds = append(seeds, i)
	}
	return seeds
}

func parseRangeInitialSeeds(s string) []uint64 {
	seeds := parseInitialSeeds(s)
	result := make([]uint64, 0)

	for i := 0; i < len(seeds); i += 2 {
		for j := seeds[i]; j < seeds[i]+seeds[i+1]; j++ {
			result = append(result, j)
		}
	}

	return result
}

// this function will be panic when the numbers slice is nil
func lowest[T ~uint64](numbers []T) T {
	if len(numbers) > 1 {
		n := numbers[0]
		for _, m := range numbers[1:] {
			if m < n {
				n = m
			}
		}
		return n
	}
	return numbers[0]
}
