use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    println!("Hello, Lets start a war!");

    let mut game = Game::new();
    game.play();
}

#[derive(Debug)]
struct Player {
    hand: Vec<Card>,
}

#[derive(Debug)]
struct Game {
    computer_player: Player,
    human_player: Player,
}

// Shuffle the deck
// Ensure the deck is ordered
fn shuffle_deck(deck: &mut Vec<Card>) {
    //TODO: Currently this is a naive shuffle, we should implement a better shuffle
    let mut second_half = deck.split_off(deck.len() / 2);
    let mut first_half = deck.split_off(0);
    deck.append(&mut second_half);
    deck.append(&mut first_half);
}

impl Game {
    fn new() -> Game {
        let mut deck = Deck::new();

        // Shuffle the deck
        shuffle_deck(&mut deck.cards);

        // Deal the cards as a human would
        /*
        let mut card = deck.cards.pop();
        while card.is_some() {
            if (deck.cards.len() % 2) == 0 {
                computer_hand.push(card.unwrap());
            } else {
                human_hand.push(card.unwrap());
            }
            card = deck.cards.pop();
        }
        */

        // Deal the cards as twitch chat would
        let player_hand = deck.cards.split_off(deck.cards.len() / 2);
        let computer_hand = deck.cards.split_off(0);
        println!("Player Hand: {:?}", player_hand);
        println!("Computer Hand: {:?}", computer_hand);

        Game {
            computer_player: Player {
                hand: computer_hand,
            },
            human_player: Player { hand: player_hand },
        }
    }

    fn play(&mut self) {
        while !self.human_player.hand.is_empty() && !self.computer_player.hand.is_empty() {
            self.play_round();
        }

        // Determine the winner
        if self.human_player.hand.is_empty() {
            println!("Computer wins!");
        } else {
            println!("Player wins!");
        }
    }

    fn play_round(&mut self) {
        // Compare the cards
        // The player with the higher card wins
        // If the cards are the same, the player with the higher suit wins
        // If the suits are the same, the game is a draw
        // If the game is a draw, the cards are added to the bottom of the deck
        // If the game is not a draw, the winner takes the cards
        // The game continues until one player has all the cards
        // The player with all the cards wins
        // The game is a draw if the game continues for too long
        // Draw a card from each player's hand
        let player_card = self.human_player.hand.pop().unwrap();
        let computer_card = self.computer_player.hand.pop().unwrap();

        println!(
            "Player card: {:?} vs {:?} Computer Card",
            player_card, computer_card
        );

        // Compare the cards
        match player_card.value.cmp(&computer_card.value) {
            std::cmp::Ordering::Equal => {
                // TODO: Draw, have a WAR.
            }
            std::cmp::Ordering::Greater => {
                self.human_player.hand.push(player_card);
                self.human_player.hand.push(computer_card);
            }
            std::cmp::Ordering::Less => {
                self.computer_player.hand.push(player_card);
                self.computer_player.hand.push(computer_card);
            }
        }
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    value: CardValue,
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
enum CardValue {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Deck {
    fn new() -> Deck {
        let mut cards = Vec::with_capacity(52);
        for suit in Suit::iter() {
            for value in CardValue::iter() {
                cards.push(Card { suit, value });
            }
        }
        Deck { cards }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_shuffle_deck() {
        let mut deck = Deck::new();
        let original_deck = deck.cards.clone();
        shuffle_deck(&mut deck.cards);

        for i in 0..deck.cards.len() {
            assert_ne!(deck.cards[i], original_deck[i]);
        }
    }

    #[test]
    fn test_shuffle_two_decks_different() {
        let mut deck = Deck::new();
        let mut deck2 = Deck::new();
        shuffle_deck(&mut deck.cards);
        shuffle_deck(&mut deck2.cards);

        for i in 0..deck.cards.len() {
            assert_ne!(deck.cards[i], deck2.cards[i]);
        }
    }

    #[test]
    fn test_war() {
        let mut game = Game::new();
        game.human_player.hand = vec![
            Card {
                suit: Suit::Spades,
                value: CardValue::Ace,
            },
            Card {
                suit: Suit::Spades,
                value: CardValue::Two,
            },
        ];
        game.computer_player.hand = vec![
            Card {
                suit: Suit::Hearts,
                value: CardValue::Ace,
            },
            Card {
                suit: Suit::Hearts,
                value: CardValue::Two,
            },
        ];

        game.play_round();

        assert_eq!(game.human_player.hand.len(), 0);
        assert_eq!(game.computer_player.hand.len(), 4);
    }
}
