package main

import (
	"fmt"
	"sort"

	"golang.org/x/exp/constraints"
)

func main() {
	f := []float64{4.2, 2.9, 5.32, 1.70, 3.65}
	sortAny(f)
	printAnySorted(f)

	i := []int{6, 5, 1, 3, 4, 2}
	sortAny(i)
	printAnySorted(i)

	s := []string{"He", "She", "They", "It", "We"}
	sortAny(s)
	printAnySorted(s)

	u := []MyString{"C", "D", "B", "A", "E"}
	sortAny(u)
	printAnySorted(u)

	// error type []int of i does not match inferred type []float64 for []T
	// printAnySorted(f, i)

	fmt.Println(equal("hello", "world!"))

	n := node[string]{val: "test"}
	fmt.Println(n)
}

type node[T constraints.Ordered] struct {
	val T
}

func equal[K comparable](i, j K) bool {
	return i == j
}

func printAnySorted[T constraints.Ordered](t ...[]T) {
	for _, v := range t {
		fmt.Println(v)
	}
}

func sortAny[T constraints.Ordered](t []T) {
	sort.Slice(t, func(i, j int) bool { return t[i] < t[j] })
}

type MyString string
