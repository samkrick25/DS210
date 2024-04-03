import datetime as dt
import random as rand

def BirthdayGenerator(value):
    birthdays = []
    for _ in range(value):
        start = dt.date(2024, 1, 1)
        birthday = start + dt.timedelta(days=rand.randint(0,364))
        birthdays.append(birthday)
    return birthdays

def matchingBirthdays(birthdays):
    bdayarray = []
    if len(birthdays) == len(set(birthdays)):
        return bdayarray
    for a, birthday1 in enumerate(birthdays):
        bdayarray.extend(
            birthday1
            for birthday2 in birthdays[a + 1 :]
            if birthday1 == birthday2
        )
    
    return bdayarray

print("""
The Birthday Paradox, by Al Swiegart, written by Sam Krick. Have you ever thought about how many people in a group share a birthday?
How common is that? This program lets you input the number of people in a group (max 100), and will run 100,000 simulations to 
see the chance that two people in that group share a birthday. It may be higher than you expect! Or will it be lower?
""")

MONTHS = ('Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul',
'Aug', 'Sept', 'Oct', 'Nov','Dec')

while True:
    print('How many birthdays should I generate? (Max 100)')
    userin = input('> ')
    if userin.isdecimal() and (1 <= int(userin) <= 100):
        numBdays = int(userin)
        break
print()

print(f'Here are {numBdays} birthdays:')
birthdays = BirthdayGenerator(numBdays)
bDaystrings = [
    ((MONTHS[birthday.month-1]), birthday.day)
    for birthday in birthdays
]

for i, birthday in enumerate(bDaystrings):
    print(f'{birthday[0]} {birthday[1]}', end='')
    if i != numBdays-1:
        print(', ', end='')
print()

bdaymatch = matchingBirthdays(birthdays)
matchstrings = [
    ((MONTHS[birthday.month-1]), str(birthday.day))
    for birthday in bdaymatch
]
while True:
    if not matchstrings:
        print('There are no matching birthdays in this group!')
    elif len(matchstrings) == 1:
        string = ''.join(f'{month} {day}' for month, day in matchstrings)
        print(f'In this group, there is a matching birthday on {string}')
    else:
        string = ', '.join(f'{month} {day}' for month, day in matchstrings)
        print(f'In this group, there is a matching birthday on {string}')
    break
print()

while True:
    print(f'''Let\'s turn it up a notch! How about we simulate a group of {numBdays} people
a large number of times! Pick a number between 10,000 and 100,000, in multiples of 10,000!''')
    siminput = input('> ')
    if siminput.isdecimal and (10_000 <= int(siminput) <= 100_000) and int(siminput)%10_000==0:
        simulations = int(siminput)
        break
print()
print(f'Great! Let\'s run {simulations} simulations!')

matches = []
for i in range(simulations):
    if i % 10_000 == 0:
        print(f'{i} simulations run...')
    birthdays = BirthdayGenerator(numBdays)
    samebday = matchingBirthdays(birthdays)
    matchlen = len(matches)
    if len(samebday) != None:
        for i in range(len(samebday)):
            if len(samebday) > len(matches):
                diff = len(samebday) - len(matches)
                for _ in range(diff):
                    matches.append(0)
                    matches[-1] += 1
            else:
                matches[i] += 1
print(f'{simulations} simulations run!')

probabilities = [
    (matches[i] / simulations * 100) for i in range(len(matches))
]
print(f'''Out of {simulations} simulations of {numBdays} people,
''', end='')
for i in range(len(probabilities)):
    if i == len(probabilities)-1:
        print('and ', end='')
    if i == 0:
        print(f'There was {i+1} matching birthday {matches[i]} time ({round(probabilities[i], 2)}%)!')
    else:
        print(f'There was {i+1} matching birthdays {matches[i]} times ({round(probabilities[i], 2)}%)!')