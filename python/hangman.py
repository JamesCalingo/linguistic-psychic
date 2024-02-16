import random
import string
import urllib.request

word_site = "https://www.mit.edu/~ecprice/wordlist.10000"
response = urllib.request.urlopen(word_site)
long_txt = response.read().decode()
words = long_txt.splitlines()

print("Guess the word.")
used_letters = []
answer = random.choice(words).upper()
display = ""

for char in answer:
    if char in string.ascii_letters:
        display += "_"
    else:
        display += char
display_list = list(display)
guesses = 0
max_guess = 10

print(display)
while guesses < max_guess:
   current_guess = input().upper()
   # Note the use of .upper() here. Without it, upper and lower case letters are considered different characters, so if the answer was "A" but you typed in "a", it'd be considered wrong.

   if len(current_guess) != 1:
    print("You seem to have entered more (or less) than one character there. Try again.")
   elif current_guess not in string.ascii_uppercase:
    print("That's not a letter! Try again.")
   elif current_guess in used_letters:
    print("You've guessed that already. Try again.")
   else:
    if current_guess not in answer:
       print(f"This puzzle does not have the letter {current_guess}.")
       guesses += 1
       used_letters.append(current_guess)
       print(f"Remaining chances: {max_guess - guesses}")
       print("Your previously guessed letters: ", [letter for letter in used_letters if letter not in answer])
    else:
       print(f"We have at least one {current_guess}!")
       used_letters.append(current_guess)
    for i in range (len(answer)):
        if answer[i] == current_guess:
            display_list[i] = current_guess
            display = "".join(display_list)
    print(display)
    if display == answer:
        print("You got it!")
        break

#Only to run if the game is lost
if guesses == max_guess:
  print(f"Game over! The answer was {answer}")
