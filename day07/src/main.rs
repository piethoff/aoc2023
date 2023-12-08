use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();

    // first task
    let total_winnings = get_total_winnings(&input);
    println!("Total winnings: {total_winnings}");

    // second task
    let total_winnings_jack = get_total_winnings_jack(&input);
    println!("Total winnings with jack rule: {total_winnings_jack}");
}

fn get_total_winnings(input: &str) -> u64 {
    let mut hands_n_bids = input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let hand = line_iter.next().unwrap()
                .parse::<Hand>().unwrap();
            let bid = line_iter.next().unwrap()
                .parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();
    hands_n_bids.sort_unstable_by(|x, y| x.0.cmp(&y.0));
    hands_n_bids.iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1u64) * bid )
        .sum()
}

fn get_total_winnings_jack(input: &str) -> u64 {
    let mut hands_n_bids = input.split('\n')
        .filter(|line| line.len() != 0)
        .map(|line| {
            let mut line_iter = line.split_whitespace();
            let hand = line_iter.next().unwrap()
                .parse::<Hand2>().unwrap();
            let bid = line_iter.next().unwrap()
                .parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();
    hands_n_bids.sort_unstable_by(|x, y| x.0.cmp(&y.0));
    hands_n_bids.iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u64 + 1u64) * bid )
        .sum()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    Highest,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: [Card; 5],
}
impl Hand {
    fn hand_type(&self) -> HandType {
        let mut list = [0u32; 13];
        for card in self.cards.iter() {
            match card {
                Card::Ace => list[0] += 1,
                Card::King => list[1] += 1,
                Card::Queen => list[2] += 1,
                Card::Jack => list[3] += 1,
                Card::Ten => list[4] += 1,
                Card::Nine => list[5] += 1,
                Card::Eight => list[6] += 1,
                Card::Seven => list[7] += 1,
                Card::Six => list[8] += 1,
                Card::Five => list[9] += 1,
                Card::Four => list[10] += 1,
                Card::Three => list[11] += 1,
                Card::Two => list[12] += 1,
            }
        }

        list.sort_unstable_by(|x, y| y.cmp(x));

        match (list[0], list[1]) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPairs,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::Highest,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_ordering = self.hand_type().cmp(&other.hand_type());
        match type_ordering {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in std::iter::zip(self.cards.iter(), other.cards.iter()) {
                    let card_ordering = self_card.cmp(other_card);
                    if card_ordering != std::cmp::Ordering::Equal {
                        return Some(card_ordering);
                    }
                }
                Some(std::cmp::Ordering::Equal)
            },
            _ => Some(type_ordering),
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Card>, _>>()
            .unwrap();

        if cards.len() != 5 {
            return Err("Invalid String: {s}");
        }
        let mut cards_array = [Card::Ace; 5];
        for i in 0..5 {
            cards_array[i] = cards[i];
        }

        Ok(Self {cards: cards_array})
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum Card {
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
    Ace,
}
impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("Invalid char"),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand2 {
    cards: [Card2; 5],
}
impl Hand2 {
    fn hand_type(&self) -> HandType {
        let mut list = [0u32; 13];
        let mut jacks = 0u32;
        for card in self.cards.iter() {
            match card {
                Card2::Ace => list[0] += 1,
                Card2::King => list[1] += 1,
                Card2::Queen => list[2] += 1,
                // Card::Jack => list[3] += 1,
                Card2::Ten => list[4] += 1,
                Card2::Nine => list[5] += 1,
                Card2::Eight => list[6] += 1,
                Card2::Seven => list[7] += 1,
                Card2::Six => list[8] += 1,
                Card2::Five => list[9] += 1,
                Card2::Four => list[10] += 1,
                Card2::Three => list[11] += 1,
                Card2::Two => list[12] += 1,
                Card2::Jack => jacks += 1,
            }
        }

        list.sort_unstable_by(|x, y| y.cmp(x));
        list[0] += jacks;

        match (list[0], list[1]) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, 2) => HandType::TwoPairs,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::Highest,
        }
    }
}
impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let type_ordering = self.hand_type().cmp(&other.hand_type());
        match type_ordering {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in std::iter::zip(self.cards.iter(), other.cards.iter()) {
                    let card_ordering = self_card.cmp(other_card);
                    if card_ordering != std::cmp::Ordering::Equal {
                        return Some(card_ordering);
                    }
                }
                Some(std::cmp::Ordering::Equal)
            },
            _ => Some(type_ordering),
        }
    }
}
impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl FromStr for Hand2 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<Card2>, _>>()
            .unwrap();

        if cards.len() != 5 {
            return Err("Invalid String: {s}");
        }
        let mut cards_array = [Card2::Ace; 5];
        for i in 0..5 {
            cards_array[i] = cards[i];
        }

        Ok(Self {cards: cards_array})
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum Card2 {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    // Jack,
    Queen,
    King,
    Ace,
}
impl TryFrom<char> for Card2 {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("Invalid char"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = 
            "32T3K 765\n\
             T55J5 684\n\
             KK677 28\n\
             KTJJT 220\n\
             QQQJA 483";
        let total_winnings = 6440;
        assert_eq!(get_total_winnings(&input), total_winnings);
    }

    #[test]
    fn test_example2() {
        let input = 
            "32T3K 765\n\
             T55J5 684\n\
             KK677 28\n\
             KTJJT 220\n\
             QQQJA 483";
        let total_winnings = 5905;
        assert_eq!(get_total_winnings_jack(&input), total_winnings);
    }
}
