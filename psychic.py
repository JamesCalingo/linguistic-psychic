import random
import string

print("Guess a letter within ten tries.")
usedLetters = []
answer = random.choice(string.ascii_uppercase)
guesses = 0
maxGuess = 10
# print(answer) - I had this here for testing, but you probably wouldn't want this for the actual game unless youre a CHEATER!
while guesses < maxGuess:
   currentGuess = input().upper()
   if currentGuess == answer:
     print("You win!!")
     break
   elif len(currentGuess) > 1:
     print("You, uh, seem to have entered more than one character there.\nI can promise you you're only looking for a single letter.")
   elif currentGuess not in string.ascii_uppercase:
     print("That's not a letter!")
   elif currentGuess in usedLetters:
     print("You've guessed that already!\nSpoiler alert: it's not the right answer.")
   else:
      guesses += 1
      usedLetters.append(currentGuess.upper())
      print(f"Incorrect. You have {maxGuess - guesses} more chances")

print(f"The answer was {answer}")
