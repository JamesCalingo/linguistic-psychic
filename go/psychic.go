package main

import (
	"fmt"
	"math/rand"
	"slices"
	"strings"
)

func main() {
	letters := []string{"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}
	guesses := make([]string, 0)
	var guess string
	chances := 9
	answer := strings.ToUpper(letters[rand.Intn(len(letters))])
	// var previouslyGuessed bool
	fmt.Println("Guess the letter. You have ten tries. Good luck!")
	for chances >= 0 {
		fmt.Print("Input a guess: ")
		fmt.Scan(&guess)
		if len(guess) != 1 || !slices.Contains(letters, strings.ToLower(guess)) {
			fmt.Println("This is an invalid input. Try again.")
		} else if chances == 0 {
			fmt.Println("Game Over! The answer was", answer)
			break
		} else if strings.ToUpper(guess) == answer {
			fmt.Println("You got it!")
			break
		} else {
			fmt.Println("Wrong")
			chances -= 1
			guesses = append(guesses, guess)
			fmt.Println("Guesses remaining:", chances+1)
			// previouslyGuessed = slices.Contains(guesses, guess)
			// if previouslyGuessed {
			// 	fmt.Println(previouslyGuessed, guesses)
			// }
		}
	}
}
