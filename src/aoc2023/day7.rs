use std::collections::HashMap;

use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let mut hands: Vec<Hand> = input.trim_end().split('\n').map(Hand::parse).collect();

        hands.sort();

        let mut out = 0;
        for (index, hand) in hands.iter().enumerate() {
            out += hand.bet() * (index + 1) as u64;
        }

        Ok(out.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let mut hands: Vec<Hand> = input.trim_end().split('\n').map(Hand::parse2).collect();

        hands.sort();

        let mut out = 0;
        for (index, hand) in hands.iter().enumerate() {
            out += hand.bet() * (index + 1) as u64;
        }

        Ok(out.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Joker,
    Queen,
    King,
    Ace,

    TwoTwo,
    ThreeTwo,
    FourTwo,
    FiveTwo,
    SixTwo,
    SevenTwo,
    EightTwo,
    NineTwo,
    TTwo,
    JokerTwo,
    QueenTwo,
    KingTwo,
    AceTwo,
}

impl Card {
    fn card_value(&self) -> u8 {
        match self {
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::T => 10,
            Card::Joker => 11,
            Card::Queen => 12,
            Card::King => 13,
            Card::Ace => 14,

            Card::TwoTwo => 2,
            Card::ThreeTwo => 3,
            Card::FourTwo => 4,
            Card::FiveTwo => 5,
            Card::SixTwo => 6,
            Card::SevenTwo => 7,
            Card::EightTwo => 8,
            Card::NineTwo => 9,
            Card::TTwo => 10,
            Card::JokerTwo => 1,
            Card::QueenTwo => 12,
            Card::KingTwo => 13,
            Card::AceTwo => 14,
        }
    }

    fn from_char(s: char, pt: bool) -> Option<Self> {
        match (s, pt) {
            ('2', false) => Some(Card::Two),
            ('3', false) => Some(Card::Three),
            ('4', false) => Some(Card::Four),
            ('5', false) => Some(Card::Five),
            ('6', false) => Some(Card::Six),
            ('7', false) => Some(Card::Seven),
            ('8', false) => Some(Card::Eight),
            ('9', false) => Some(Card::Nine),
            ('T', false) => Some(Card::T),
            ('J', false) => Some(Card::Joker),
            ('Q', false) => Some(Card::Queen),
            ('K', false) => Some(Card::King),
            ('A', false) => Some(Card::Ace),
            ('2', true) => Some(Card::TwoTwo),
            ('3', true) => Some(Card::ThreeTwo),
            ('4', true) => Some(Card::FourTwo),
            ('5', true) => Some(Card::FiveTwo),
            ('6', true) => Some(Card::SixTwo),
            ('7', true) => Some(Card::SevenTwo),
            ('8', true) => Some(Card::EightTwo),
            ('9', true) => Some(Card::NineTwo),
            ('T', true) => Some(Card::TTwo),
            ('J', true) => Some(Card::JokerTwo),
            ('Q', true) => Some(Card::QueenTwo),
            ('K', true) => Some(Card::KingTwo),
            ('A', true) => Some(Card::AceTwo),
            _ => None,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.card_value().partial_cmp(&other.card_value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.card_value().cmp(&other.card_value())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    PtOne(HandCards),
    PtTwo(HandCards),
}

#[derive(Debug, PartialEq, Eq)]
struct HandCards {
    cards: [Card; 5],
    bet: u64,
}

impl Hand {
    fn cards(&self) -> &[Card; 5] {
        match self {
            Hand::PtOne(cards) => &cards.cards,
            Hand::PtTwo(cards) => &cards.cards,
        }
    }

    fn bet(&self) -> &u64 {
        match self {
            Hand::PtOne(cards) => &cards.bet,
            Hand::PtTwo(cards) => &cards.bet,
        }
    }

    fn hand_value(&self) -> u8 {
        let mut values = HashMap::new();

        for card in self.cards().iter() {
            let count = values.entry(card).or_insert(0);
            *count += 1;
        }

        match self {
            Hand::PtOne(_) => {
                if values.iter().any(|(_, c)| *c == 5) {
                    return 7;
                }

                if values.iter().any(|(_, c)| *c == 4) {
                    return 6;
                }

                if values.iter().any(|(_, c)| *c == 3) {
                    return if values.iter().any(|(_, c)| *c == 2) {
                        5
                    } else {
                        4
                    };
                }

                match values.iter().filter(|(_, c)| **c == 2).count() {
                    2 => return 3,
                    1 => return 2,
                    _ => {}
                }

                if values.iter().all(|(_, c)| *c == 1) {
                    1
                } else {
                    0
                }
            }
            Hand::PtTwo(_) => {
                let jokers = values.get(&Card::JokerTwo).unwrap_or(&0);
                // println!("Jokers: {jokers}");
                // println!("Cards: {values:?}");

                fn jokerzero(card: &&Card, jokers: &i32) -> i32 {
                    if matches!(card, Card::JokerTwo) {
                        0
                    } else {
                        *jokers
                    }
                }

                if values
                    .iter()
                    .any(|(card, c)| *c >= 5 - jokerzero(card, jokers))
                {
                    return 7;
                }

                if values
                    .iter()
                    .any(|(card, c)| *c >= 4 - jokerzero(card, jokers))
                {
                    return 6;
                }

                if let Some((three_cards, _)) = values
                    .iter()
                    .find(|(card, c)| **c >= 3 - jokerzero(card, jokers))
                {
                    return if values.iter().any(|(card, c)| {
                        three_cards != card && !matches!(card, Card::JokerTwo) && *c >= 2
                    }) {
                        5
                    } else {
                        4
                    };
                }

                if let Some((two_cards, _)) = values
                    .iter()
                    .find(|(card, c)| **c >= 2 - jokerzero(card, jokers))
                {
                    return if values.iter().any(|(card, c)| {
                        two_cards != card && !matches!(card, Card::JokerTwo) && *c >= 2
                    }) {
                        3
                    } else {
                        2
                    };
                }

                if values
                    .iter()
                    .all(|(card, c)| matches!(**card, Card::JokerTwo) || *c == 1)
                {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn parse(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let cards = split
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| Card::from_char(c, false))
            .collect::<Vec<Card>>();
        let bet = split.next().unwrap().parse().unwrap();

        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        Self::PtOne(HandCards { cards, bet })
    }

    fn parse2(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let cards = split
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| Card::from_char(c, true))
            .collect::<Vec<Card>>();
        let bet = split.next().unwrap().parse().unwrap();

        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];

        Self::PtTwo(HandCards { cards, bet })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value = self.hand_value();
        let other_value = other.hand_value();

        if self_value != other_value {
            return self_value.partial_cmp(&other_value);
        } else {
            for (self_card, other_card) in self.cards().iter().zip(other.cards().iter()) {
                if self_card != other_card {
                    return self_card.partial_cmp(other_card);
                }
            }

            return Some(std::cmp::Ordering::Equal);
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = self.hand_value();
        let other_value = other.hand_value();

        if self_value != other_value {
            return self_value.cmp(&other_value);
        } else {
            for (self_card, other_card) in self.cards().iter().zip(other.cards().iter()) {
                if self_card != other_card {
                    return self_card.cmp(other_card);
                }
            }
            return std::cmp::Ordering::Equal;
        }
    }
}

#[cfg(test)]
mod test_pt2 {
    use super::*;

    #[test]
    fn hands_weight_should_be_five_of_kind() {
        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 7);

        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::JokerTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 7);

        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::JokerTwo,
                Card::JokerTwo,
                Card::JokerTwo,
                Card::JokerTwo,
                Card::JokerTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 7);
    }

    #[test]
    fn hands_weight_should_be_four_of_kind() {
        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::TwoTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 6);

        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::TwoTwo,
                Card::JokerTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 6);

        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::JokerTwo,
                Card::JokerTwo,
                Card::JokerTwo,
                Card::ThreeTwo,
                Card::TwoTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 6);
    }

    #[test]
    fn hands_weight_should_be_house() {
        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::TwoTwo,
                Card::TwoTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 5);

        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::AceTwo,
                Card::TwoTwo,
                Card::TwoTwo,
                Card::JokerTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 5);
    }

    #[test]
    fn hands_weight_should_be_three() {
        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::AceTwo,
                Card::AceTwo,
                Card::ThreeTwo,
                Card::TwoTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 4);

        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::ThreeTwo,
                Card::TwoTwo,
                Card::TwoTwo,
                Card::JokerTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 4);

        let hand = Hand::PtTwo(HandCards {
            cards: [
                Card::AceTwo,
                Card::ThreeTwo,
                Card::TwoTwo,
                Card::JokerTwo,
                Card::JokerTwo,
            ],
            bet: 0,
        });

        assert_eq!(hand.hand_value(), 4);
    }
}
