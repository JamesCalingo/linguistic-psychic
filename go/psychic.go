package main

import (
	"fmt"
	"math/rand"
	"slices"
	"strings"
)

func main() {
	letters := []string{"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}
	var guess string
	chances := 9
	answer := strings.ToUpper(letters[rand.Intn(len(letters))])
	for chances >= 0 {
		fmt.Print("Input a guess: ", answer)
		fmt.Scan(&guess)
		if len(guess) != 1 {
			fmt.Println("This is an invalid input. Try again.")
		} else if !slices.Contains(letters, strings.ToLower(guess)) {
			fmt.Println("This isn't a letter. Try again.")
		} else if chances == 0 {
			fmt.Println("Game Over! The answer was:", answer)
			break
		} else if strings.ToUpper(guess) == answer {
			fmt.Println("You got it!")
			break
		} else {
			fmt.Println("Wrong")
			chances -= 1
			fmt.Println("Chances remaining:", chances+1)
		}
	}
}
