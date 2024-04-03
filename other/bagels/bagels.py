'''
Bagels, a numbers game, written by Sam Krick and inspired by Al Swiegart
BigBookPython
Guess the number with clues after each guess!
'''
import random

GUESSES = 10
NUMLENGTH = 3

def main():
    print('''
    Bagels, a logic game from Al Swiegart written by Sam Krick
    I will think of a {}-digit number. You will have {} attempts to guess this number.
    This number will not repeat any digits
    After you input your guess, I will give you clues.
    Bagels means that no digits in your guess were correct.
    Pico means that one digit in your guess is correct, but in the wrong place.
    Fendi means that one digit in your guess is correct, and in the right place.
    '''.format(NUMLENGTH, GUESSES))
    while True:
        secretNum = SecretNumGen()
        print(f'I have thought of a {NUMLENGTH}-digit number, you have {GUESSES} guesses to get it:')
        guesscount = 1
        while guesscount <= GUESSES:
            guess = ' '
            print(f'Guess #{guesscount}:')
            guess = input('Your guess: ')
            clues = GetClues(guess, secretNum)
            print(f"{clues}")
            guesscount += 1
            if guess == secretNum:
                break
            if guesscount > GUESSES:
                print('You have exceeded the maximum number of guesses')
                print(f'The secret number was {secretNum}')
                break
        print('Would you like to play again? y/n')
        if not input('> ').lower().startswith('y'):
            break
    print('Thanks for playing!') 

def SecretNumGen():
    secretNumlist = random.sample(range(10),3)
    return ''.join(str(secretNumlist[i]) for i in range(len(secretNumlist)))

def GetClues(guess, secretNum):
    if guess == secretNum:
        return 'You got it!'
    clues = []
    for i in range(len(secretNum)):
        if secretNum[i] == guess[i]:
            clues.append('Fendi ')
        elif guess[i] in secretNum:
            clues.append('Pico ')
        else:
            continue
    if not clues:
        clues.append('Bagels')
    else:
        clues.sort()
    return ''.join(clues)

if __name__ == '__main__':
    main()
    