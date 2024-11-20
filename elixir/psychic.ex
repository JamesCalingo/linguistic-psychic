letters = [
  "a",
  "b",
  "c",
  "d",
  "e",
  "f",
  "g",
  "h",
  "i",
  "j",
  "k",
  "l",
  "m",
  "n",
  "o",
  "p",
  "q",
  "r",
  "s",
  "t",
  "u",
  "v",
  "w",
  "x",
  "y",
  "z"
]

defmodule Gameloop do

  def check(guess, answer) do
    chances = 10
    cond do
      String.length(guess) != 2 ->
        guess = IO.gets("This is not a valid guess. Try again: ")
        Gameloop.check(guess, answer)

      String.upcase(guess) == answer ->
        IO.puts("You got it!")

      true ->
        guess = IO.gets("Nope. Guess again: ")
        IO.puts("Chances remaining: " <> to_string(chances))
        Gameloop.check(guess, answer)
        chances = chances - 1
    end
  end
end

# Concatenating this is the only way to ensure correct checking due to IO.gets default behavior
answer = String.upcase(Enum.random(letters)) <> "\n"
# IO.puts(answer)
guess = IO.gets("Guess the letter: ")
Gameloop.check(guess, answer)
