package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
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

	re, err := regexp.Compile("(one|two|three|four|five|six|seven|eight|nine|\\d)(?:.*(one|two|three|four|five|six|seven|eight|nine|\\d))?")
	if err != nil {
		panic(err)
	}

	reader := bufio.NewScanner(file)

	for reader.Scan() {
		line := reader.Text()
		fmt.Println(line)
		submatches := re.FindStringSubmatch(line)
		if len(submatches) == 0 {
			panic("no match")
		}
		first_digit := submatches[1]
		second_digit := submatches[2]
		if second_digit == "" {
			second_digit = first_digit
		}
		fmt.Println(first_digit, second_digit)
	}

}
