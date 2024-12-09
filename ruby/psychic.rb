letters = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]
previous = []
answer = letters[rand(26)]

puts "Guess the letter!"
# puts answer

solved = false

chances = 10

while chances > 0
    guess = gets.strip.upcase
    if !letters.include? guess || guess.length > 1
        puts "That is not an eligible guess."
    elsif previous.include? guess
        puts "You've guessed this already."
    elsif answer.strip.upcase == guess
        puts "You Win!"
        solved = true
        break
    else
        puts "Nope"
        previous.push(guess)
        chances -= 1
        puts "Chances remaining: " + chances.to_s
    end
end

if !solved
    puts "Game over...the answer was " + answer
end
