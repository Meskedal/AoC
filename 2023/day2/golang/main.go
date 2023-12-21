package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Printf("Usage: %s <input>\n", os.Args[0])
		return
	}

	input := os.Args[1]

	all, err := os.ReadFile(input)
	if err != nil {
		panic(err)
	}

	fmt.Println(string(all))
	file, err := os.Open(input)
	if err != nil {
		panic(err)
	}

	re, err := regexp.Compile("(?:Game (\\d*))|(?:(\\d*) red)|(?:(\\d*) green)|(?:(\\d*) blue)")
	if err != nil {
		panic(err)
	}

	reader := bufio.NewScanner(file)

	var (
		maxRed   int64 = 12
		maxGreen int64 = 13
		maxBlue  int64 = 14
	)

	maxValues := []int64{maxRed, maxGreen, maxBlue}

	var sumPart1 int64 = 0
	var sumPart2 int64 = 0

	for reader.Scan() {
		line := reader.Text()
		fmt.Println(line)
		submatches := re.FindAllStringSubmatch(line, -1)
		if len(submatches) == 0 {
			panic("no match")
		}
		game := submatches[0][1]
		gameInt, err := strconv.ParseInt(game, 10, 64)
		fmt.Println(gameInt)
		if err != nil {
			fmt.Println("game", game)
			panic(err)
		}
		valid := true
		minValues := []int64{-1, -1, -1}
		for _, submatch := range submatches[1:] {
			fmt.Println(submatch[2:])
			for colourIndex, val := range submatch[2:] {
				if val == "" {
					continue
				}
				valInt, err := strconv.ParseInt(val, 10, 64)
				if err != nil {
					panic(err)
				}
				if valInt > minValues[colourIndex] {
					minValues[colourIndex] = valInt
				}
				if valInt > maxValues[colourIndex] {
					fmt.Println(colourIndex, valInt, maxValues[colourIndex])
					fmt.Printf("Game %d, %s: %d > %d\n", gameInt, submatch, valInt, maxValues[colourIndex])
					valid = false
					continue
				}

			}
		}
		sumPart2 += minValues[0] * minValues[1] * minValues[2]

		if !valid {
			continue
		}

		sumPart1 += gameInt
	}
	fmt.Println(sumPart1)
	fmt.Println(sumPart2)

}
