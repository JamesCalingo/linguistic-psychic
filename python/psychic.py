import random
import string

print("Guess a letter within ten tries.")
used_letters = []
answer = random.choice(string.ascii_uppercase)
guesses = 0
max_guess = 10
# print(answer) - I had this here for testing, but you probably wouldn't want this for the actual game unless youre a CHEATER!
while guesses < max_guess:
   current_guess = input().upper()
   # Note the use of .upper() here. Without it, upper and lower case letters are considered different characters, so if the answer was "A" but you typed in "a", it'd be considered wrong.
   if current_guess == answer:
     print("You got it!")
     break
   elif len(current_guess) != 1:
     print("You seem to have entered more (or less) than one character there. Try again.")
   elif current_guess not in string.ascii_uppercase:
     print("That's not a letter! Try again.")
   elif current_guess in used_letters:
     print("You've guessed that already. Try again.")
   else:
      guesses += 1
      used_letters.append(current_guess)
      if guesses < 9:
        print(f"Nope...you have {max_guess - guesses} more chances")
      else:
        print("Nope. Only one guess left...make it count!")

if guesses == max_guess:
  print(f"Game over! The answer was {answer}")
