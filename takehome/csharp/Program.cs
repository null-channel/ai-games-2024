// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");
var game = new Game();
game.Start();


class Card
{
    public Suit Suit { get; set; }
    public Value Value { get; set; }

    public Card(Suit suit, Value value)
    {
        Suit = suit;
        Value = value;
    }
}

class Player
{
    public List<Card> Hand { get; set; }

    public Player()
    {
        Hand = new List<Card>();
    }
}

class Game
{
    private Player player_one = new();
    private Player player_two = new();
    
    public Game()
    {
        Deck deck = new(); 

        deck.Shuffle();
        player_one.Hand = deck.Draw(deck.cards.Count / 2);
        player_two.Hand = deck.Draw(deck.cards.Count);
    }

    public void Start()
    {
        while (player_one.Hand.Count > 0 && player_two.Hand.Count > 0)
        {
            Console.WriteLine("Player one plays a card");
            Console.WriteLine("Player two plays a card");
            var player_one_card = player_one.Hand.First();
            player_one.Hand.Remove(player_one_card);
            var player_two_card = player_two.Hand.First();
            player_two.Hand.Remove(player_two_card);
            // compare the cards
            if (player_one_card.Value > player_two_card.Value)
            {
                player_one.Hand.Add(player_one_card);
                player_one.Hand.Add(player_two_card);
            } 
            else if (player_one_card.Value < player_two_card.Value)
            {
                player_two.Hand.Add(player_one_card);
                player_two.Hand.Add(player_two_card);
            }
            else
            {
                // TODO: WAR
            }
        }

        if (player_one.Hand.Count > player_two.Hand.Count)
        {
            Console.WriteLine("Player one wins!");
        }
        else
        {
            Console.WriteLine("Player two wins!");
        }
    }
}

enum Suit
{
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

enum Value
{
    Two,
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
    Ace
}

class Deck
{
    public List<Card> cards = new List<Card>();

    public Deck()
    {
        for (int i = 0; i < 4; i++)
        {
            for (int j = 0; j < 13; j++)
            {
                cards.Add(new Card((Suit)i, (Value)j));
            }
        }
    }

    //get half the deck
    public List<Card> Draw(int amount)
    {
        List<Card> drawn = cards.Take(amount).ToList();
        cards.RemoveRange(0, amount);
        return drawn;
    }

    // shuffle the deck
    // Ensure the deck is ordered first by suit then by value
    // TODO: Quick implementation needs to be improved
    public void Shuffle()
    {
        List<Card> shuffled = new List<Card>();
        // get first half of the deck
        List<Card> first_half = cards.Take(cards.Count / 2).ToList();
        // get second half of the deck
        List<Card> second_half = cards.Skip(cards.Count / 2).ToList();
        
        cards.Clear();
        cards.AddRange(second_half);
        cards.AddRange(first_half);
    }
}


