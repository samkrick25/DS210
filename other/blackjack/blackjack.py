'''Python Blackjack Bot, written by Sam Krick, inspired by Al Swiegart's blackjack program in the Big Book of Small Python Projects.
Try to get 21!'''
import sys, random, time

HEARTS   = chr(9829) # Character 9829 is '♥'.
DIAMONDS = chr(9830) # Character 9830 is '♦'.
SPADES   = chr(9824) # Character 9824 is '♠'.
CLUBS    = chr(9827) # Character 9827 is '♣'.
SUITS = {HEARTS, DIAMONDS, SPADES, CLUBS}
BACKSIDE = 'backside'
W = 21

def main():
    print('''Welcome to Blackjack! Your goal is to get cards equal to 21!
You will get dealt two cards face up. 
The dealer will get one card face down that they know and one face up that you both know.
Each card has its face value, while face cards J, Q, K, have the value 11, 12, 13 respectively. 
Aces can be either 1 or 11, depending on if they make you bust or not.
On your turn, you can either (H)it, (S)tand, or, on your first turn only, (D)ouble Down.
Hitting will draw a card, standing keeps you where you are and lets the dealer take its turn.
The dealer will stop hitting at 17.
Doubling down allows you to increase your bet by up to 500, only on your first turn.
You have $5000, don't suck!
''')
    money = 5000
    #main game loop
    while True:
        print(f'You have ${money}.')
        #if player is broke, player has lost and quit the program
        if money == 0:
            print('C\'mon, I told you don\'t suck!')
            print('You\'re out of money...')
            print('Good thing its fake! Or was it Venmo money...')
            sys.exit()
        #get player bet
        playerBet = getBet(money)
        print(f'Bet: {playerBet}')
        #get deck, and deal cards to player and dealer
        deck = getDeck()
        playerHand = [deck.pop(), deck.pop()]
        dealerHand = [deck.pop(), deck.pop()]
        #deal with player's turn, loop until player busts, stands, or hits 21
        while not getValue(playerHand) >= W: 
            showHands(dealerHand, playerHand)
            #if player hand is 21 or higher, they don't make any more moves
            if getValue(playerHand) > W:
                #player has bust, so break
                break
            #player hand is under 21, so play
            if getValue(playerHand) < W:
                move = getMove(playerHand, playerBet, money)
                match move :
                    case 'H':
                        playerHand.append(getNewCard(deck))
                    case 'D':
                        print('Ah, doubling down I see...Leon I told you to not use my blackjack bot for practice anymore!')
                        print('Place another bet! You can only bet up to 500, or the maximum amount of money you have left!')
                        if money < 500:
                            playerBet = getBet(money)
                        if money >= 500:
                            playerBet = getBet(500)
                        playerHand.append(getNewCard(deck))
                    case 'S':
                        #standing ends player turn
                        break
                    case _:
                        raise ValueError('Shit! getMove() is broken, it returned smth other than S, D, or H')
        if getValue(playerHand) > W:
            print('Busted!')
            time.sleep(1)
            print('...mmh...')
            time.sleep(1)
            print('You lost your money! Try again.')
            playerHand.clear()
            dealerHand.clear()
            deck.clear()
            money = money - playerBet
            continue
                
        time.sleep(1.5)
        #dealer turn
        showHands(dealerHand, playerHand)
        while getValue(dealerHand) < 17:
            print('Dealer Hits...')
            dealerHand.append(getNewCard(deck))
            showHands(dealerHand, playerHand)
            if getValue(dealerHand) >= W:
                break
        playerValue = getValue(playerHand)
        dealerValue = getValue(dealerHand)
        showHands(dealerHand, playerHand, backside=False)
        if dealerValue > W:
            print('Dealer busts!...damn you got the gluck gluck 3000...')
            print('You got your bet back!')
        elif playerValue > W or dealerValue > playerValue:
            print('You lost...not poggers...')
            print('There goes your bet.')
            money -= playerBet
        elif playerValue == W and dealerValue != W:
            print('Blackjack! You win 1.5x your bet!')
            money += playerBet*1.5
        elif playerValue > dealerValue:
            print('You win! You win your bet!')
            money += playerBet
        deck.clear()
        playerHand.clear()
        dealerHand.clear()
        input('Press Enter to continue...')
        print('\n\n')
        

def getDeck() -> list: 
    """generates a list of 52 tuples where each tuple represents a card in a 52 card deck, in the form (rank, suit)

    Returns:
        list: this is the deck that cards will be 'dealt' from
    """
    deck = []
    for suit in SUITS:
        deck.extend((str(rank), suit) for rank in range(2,11))
        deck.extend((face, suit) for face in ('A', 'J', 'Q', 'K'))
    random.shuffle(deck)
    return deck
        
            

def getBet(money: int, currentbet: int = 0) -> int:
    """this function takes a users input and transforms it into a string, only allowing inputs that are equal or less to the amount entered

    Args:
        money (int): maximum bet allowed

    Returns:
        int: the player's bet
    """
    while True:
        print(f'Place your bet! Input a number from 1-{money}, or QUIT to exit')
        bet = input('> ').strip().upper()
        if bet == 'QUIT':
            sys.exit()
        if not bet.isdecimal():
            continue
        bet = int(bet)
        if 1 <= bet <= money:
            return bet

def getMove(hand: list, currentbet: int = 0, money: int = 0) -> str:
    """ask player for an input, the input has to be S, D, or H, but can only be D on the player's first turn

    Args:
        hand (list): this should be a list of tuples, which will represent the player's hand

    Returns:
        str: the move the player will make
    """
    while True:
        moves = ['(H)it', '(S)tand']
        if len(hand) == 2 and currentbet < money-500:
            moves.append('(D)ouble Down')
        prompt = ', '.join(moves) + '> '
        move = input(prompt).upper()
        if move in ('H', 'S'):
            return move
        if move == 'D' and '(D)ouble Down' in moves:
            return move
        

def showHands(dealer: list, player: list, backside: bool=True) -> None:
    """This will print both player and dealer's hands

    Args:
        dealer (list): the dealer's hand, a list of tuples where each tuple is a card
        player (list): the player's hand, formatted same as the dealer's
        backside (bool, optional): whether or not the first card of the dealer's hand will be the backside of a card or not. Defaults to True.
    """
    print()
    if backside:
        print('DEALER: ???')
        getCard([BACKSIDE] + dealer[1:])
    else:
        print(f'DEALER: {getValue(dealer)}')
        getCard(dealer)
    print()
    time.sleep(1.5)
    print(f'Player: {getValue(player)}')
    getCard(player)
    
def getCard(hand: list) -> None:
    """_summary_

    Args:
        hand (list): _description_
    """
    rows = ['', '', '', '', '']  # The text to display on each row.

    for card in hand:
        rows[0] += ' ___  '  # Print the top line of the card.
        if card == BACKSIDE:
            # Print a card's back:
            rows[1] += '|## | '
            rows[2] += '|###| '
            rows[3] += '|_##| '
        else:
            # Print the card's front:
            rank, suit = card  # The card is a tuple data structure.
            rows[1] += f'|{rank.ljust(2)} | '
            rows[2] += f'| {suit} | '.format(suit)
            rows[3] += f'|_{rank.rjust(2, '_')}| '
    # Print each row on the screen:
    for row in rows:
        print(row)
    
def getNewCard(deck: list) -> tuple:
    """this card returns a new card object to be appended to the player or dealer's hand

    Args:
        deck (list): this is the deck generated by getDeck()

    Returns:
        tuple: this tuple will represent the new card
    """
    newCard = deck.pop()
    rank, suit = newCard
    print(f'You drew a {rank} of {suit}.')
    return newCard

def getValue(hand: list) -> int:
    """gets the value of a hand, numbers are treated at face value and face cards are 10, then these are summed
    if a card is an Ace, then this function will check if the player can add 10 to their value without busting,
    and if it can will add 10 for each Ace, if not then add 1 for each Ace

    Args:
        hand (list): this should be the player's hand, formatted as a list of tuples

    Returns:
        int: this will give the value of the players hand
    """
    value = 0
    ace_count = 0
    for card in hand:
        rank, _ = card  # Ignore the suit for value calculation
        if rank in ['J', 'Q', 'K']:
            value += 10
        elif rank == 'A':
            ace_count += 1
        else:
            value += int(rank)
    # Handle Aces separately to avoid double counting
    for _ in range(ace_count):
        value += 11 if value + 11 <= W else 1
    return value

if __name__ == '__main__':
    main()