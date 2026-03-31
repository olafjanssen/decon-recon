# Decon-Recon

## Protagonist
Player is a linguistic mediator in a land with disagreeing people.

## (Mini-)Game modes
Any game mode requires investing energy and results in pseudo-random loot.

### Text deconstruction
Player is presented with a layered quote from a character. Player has to select the sentence parts that pertain to a give card. A score tally is presented at the end of the phase.

Mechanics:
- faster selection gives more points, timeout exists
- obtain points for correct answer
- lose points for wrong answer

Scoring:
- points add to mining/leveling up cards

### Card-based deconstruction
Player is presented with a layered quote from a character. Player picks one out of 3-5 cards from a selected layer and continues this until the quote is fully deconstructed. A score tally is presented at the end of the phase.

Mechanics:
- faster selection gives more points, timeout exists
- obtain points for correct answer
- lose points for wrong answer
- ending shows score compared to correct solution

Scoring:
- layer deconstruction adds profile info for a character
- complete deconstruction of a full message (could be in steps) gives a 'meaning nugget'-card opening up reconstructions

### Card-based construction
Player is presented with a core nugget and has a limited time to select the correct combination of cards (in a carousel-like combination lock) until the correct solution is found. It constructs a message to a particular character.

Mechanics:
- faster selection gives more points, timeout exists
- complete construction connects two characters

## Game Loop

- Passing time increases *energy*
- Player plays incoming text-deconstruction with *energy* to unlock-level modality cards (and obtaining loot *energy* and *keys*)
- Player uses *keys* to unlock locations (i.e. characters)
- Player plays card-based deconstruction with the available unlocked cards with *energy* (obtaining loot *energy*, *keys*) and *lotus flowers* to buy new cards for the deck (which then need to be unlocked)
- Player plays card-based construction when both players are discovered and cards are available, connecting their locations, resulting in a lot of loot.

## Datasets

### Campaign decks
- card represents a campaign
- contains a hub location
- contains a introduction
- contains a resolution
- deck is all the campaigns in the game

### Character decks
- cards represents a game character
- deck represents a campaign
- card contains list with preferential modality cards
- card has a lively profile description
- card has a location

### Substant decks
- cards represent layer-0 neutral factoids
- substants can be uttered by all characters
- substants can be accepted by all characters
- deck represents the information sphere for a particular campaign

### Modality deck
- cards represent inflections in written language
- deck represents a structural balloon in which an utterance can transform
- cards contain a layer in the structural tree
- cards contain an inflection of the modality

### Utterance decks
- card represents an utterance
- is spoken by a character
- is based on a single substant
- is baded on a chain of modality cards
