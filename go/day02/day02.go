package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

type Present struct {
	l, w, h  int
}

func main() {
	sc := bufio.NewScanner(os.Stdin)
	var presents []Present
	for sc.Scan() {
		var p Present
		fmt.Sscanf(sc.Text(), "%dx%dx%d", &p.l, &p.w, &p.h)
		presents = append(presents, p)
	}

	var p1, p2 int

	for _, p := range presents {
		l, w, h := p.l, p.w, p.h

		sides := []int{l * w, w * h, h * l}
		sort.Ints(sides)
		for _, s := range sides {
			p1 += s * 2
		}
		p1 += sides[0]

		sides = []int{l + l, w + w, h + h}
		sort.Ints(sides)
		p2 += sides[0] + sides[1] + (l * w * h)
	}

	fmt.Printf("Part1 %d\n", p1)
	fmt.Printf("Part2 %d\n", p2)
}
