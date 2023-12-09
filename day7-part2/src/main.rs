use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn power(&self) -> i8 {
        match &self {
            Card::Joker => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.power().partial_cmp(&other.power())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.power().cmp(&other.power())
    }
}

impl Eq for Card {}

impl From<char> for Card {
    fn from(letter: char) -> Self {
        match letter {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Unknown card with letter '{}'", letter),
        }
    }
}

#[derive(Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn power(&self) -> u8 {
        match self {
            HandType::HighCard => 1,
            HandType::Pair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }

    fn upgrade(&self, joker_count: u8) -> HandType {
        match self {
            HandType::HighCard => match joker_count {
                0 => HandType::HighCard,
                1 => HandType::Pair,
                2 => HandType::ThreeOfAKind,
                3 => HandType::FourOfAKind,
                4 => HandType::FiveOfAKind,
                5 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            HandType::Pair => match joker_count {
                0 => HandType::Pair,
                1 => HandType::ThreeOfAKind,
                2 => HandType::FourOfAKind,
                3 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            HandType::TwoPair => match joker_count {
                0 => HandType::TwoPair,
                1 => HandType::FullHouse,
                _ => unreachable!(),
            },
            HandType::ThreeOfAKind => match joker_count {
                0 => HandType::ThreeOfAKind,
                1 => HandType::FourOfAKind,
                2 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            HandType::FullHouse => match joker_count {
                0 => HandType::FullHouse,
                _ => unreachable!(),
            },
            HandType::FourOfAKind => match joker_count {
                0 => HandType::FourOfAKind,
                1 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
            HandType::FiveOfAKind => match joker_count {
                0 => HandType::FiveOfAKind,
                _ => unreachable!(),
            },
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.power().partial_cmp(&other.power())
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.power().cmp(&other.power())
    }
}

impl Eq for HandType {}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

fn count_cards(cards: &[Card]) -> HashMap<&Card, u8> {
    let mut counts: HashMap<&Card, u8> = HashMap::new();

    for card in cards.iter() {
        counts
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    counts
}

/// Returns the counts by cards of cards passed in parameters
fn counts_by_cards<'a>(card_counts: &'a HashMap<&Card, u8>) -> HashMap<u8, HashSet<&'a Card>> {
    let mut counts_by_cards: HashMap<u8, HashSet<&Card>> = HashMap::new();

    for (card, count) in card_counts.into_iter() {
        counts_by_cards
            .entry(*count)
            .and_modify(|matching_cards| {
                matching_cards.insert(card);
            })
            .or_insert_with(|| {
                let mut matching_cards = HashSet::new();
                matching_cards.insert(*card);
                matching_cards
            });
    }

    counts_by_cards
}

fn resolve_handtype_without_joker(counts_by_cards: &HashMap<u8, HashSet<&Card>>) -> HandType {
    // if there's 5 of the same card, FiveOfAKind
    if counts_by_cards.contains_key(&5) {
        return HandType::FiveOfAKind;
    }

    // if there's 4 of the same card, FourOfAKind
    if counts_by_cards.contains_key(&4) {
        return HandType::FourOfAKind;
    }

    // if there's 3 of the same card
    if counts_by_cards.contains_key(&3) {
        // And 2 of the same card, FullHouse
        if counts_by_cards.contains_key(&2) {
            return HandType::FullHouse;
        }

        // Otherwise, ThreeOfAKind
        return HandType::ThreeOfAKind;
    }

    // Looking for pairs
    match counts_by_cards.get(&2) {
        Some(matching_cards) => {
            // if found 2 pairs, TwoPair
            if matching_cards.len() == 2 {
                HandType::TwoPair
            } else {
                // Otherwise, just a Pair
                HandType::Pair
            }
        } // in the worst case, HighCard
        None => HandType::HighCard,
    }
}

fn resolve_handtype(cards: &[Card]) -> HandType {
    let mut card_counts = count_cards(cards);
    let joker_count = card_counts.remove(&Card::Joker).unwrap_or(0);
    let counts_by_cards = counts_by_cards(&card_counts);

    let basic_handtype = resolve_handtype_without_joker(&counts_by_cards);

    basic_handtype.upgrade(joker_count)
}

impl Hand {
    fn new(cards: Vec<Card>) -> Hand {
        let hand_type = resolve_handtype(&cards);

        Hand { cards, hand_type }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        self.cards.partial_cmp(&other.cards)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        self.cards.cmp(&other.cards)
    }
}

impl Eq for Hand {}

impl From<&str> for Hand {
    fn from(hand: &str) -> Self {
        let cards = hand.chars().map(|letter| Card::from(letter)).collect();

        Hand::new(cards)
    }
}

#[derive(Debug)]
struct PlayedHand {
    hand: Hand,
    bid: u64,
}

impl From<&str> for PlayedHand {
    fn from(line: &str) -> Self {
        let parts = line.split(" ").collect::<Vec<&str>>();

        let hand = parts.get(0).expect("Line should have a played hand");
        let hand = Hand::from(*hand);

        let bid = parts.get(1).expect("Line should have a bid");
        let bid = bid.parse::<u64>().expect("Bid should be a number");

        PlayedHand { hand, bid }
    }
}

fn ex2(input: &str) -> u64 {
    let mut played_hands = input
        .lines()
        .map(|line| PlayedHand::from(line))
        .collect::<Vec<PlayedHand>>();

    played_hands.sort_by(|a, b| a.hand.cmp(&b.hand));

    played_hands
        .iter()
        .enumerate()
        .map(|(index, played_hand)| (index as u64 + 1) * played_hand.bid)
        .sum()
}

fn main() {
    let input = include_str!("../etc/input");

    println!("{}", ex2(input));
}
