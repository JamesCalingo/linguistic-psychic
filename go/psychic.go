package main

import (
	"fmt"
	"math/rand"
	"slices"
	"strings"
)

func main() {
	letters := []string{"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"}
	guesses := make([]string, 0)
	chances := 9
	answer := letters[rand.Intn(len(letters))]

	fmt.Println("Guess the letter. You have ten tries. Good luck!")
	for chances >= 0 {
		fmt.Print("Input a guess: ")
		var guess string
		fmt.Scanln(&guess)
		if !slices.Contains(letters, strings.ToUpper(guess)) {
			fmt.Println("This is an invalid input. Try again.")
		} else if slices.Contains(guesses, strings.ToUpper(guess)) {
			fmt.Println("You've guessed this already. Try again.")
		} else if strings.ToUpper(guess) == answer {
			fmt.Println("You got it!")
			break
		} else if chances == 0 {
			fmt.Println("Game Over! The answer was", answer)
			break
		} else {
			fmt.Println("Nope. Try again.")
			chances -= 1
			guesses = append(guesses, strings.ToUpper(guess))
			fmt.Println("Your previous guesses: ", guesses)
			fmt.Println("Guesses remaining:", chances)
		}
	}
}
