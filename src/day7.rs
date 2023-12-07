use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Game {
    cards: Vec<char>,
    value_part1: Option<GameValue>,
    value_part2: Option<GameValue>,
    bid: u32,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
enum GameValue {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Game {
    pub fn new(bid: u32) -> Game {
        Game {
            cards: Vec::new(),
            value_part1: None,
            value_part2: None,
            bid,
        }
    }

    pub fn add_card(&mut self, c: char) {
        self.cards.push(c);
        self.value_part1 = None;
        self.value_part2 = None;
    }

    pub fn get_value_1(&self) -> GameValue {
        self.calculate_value_1()
    }

    pub fn get_value_2(&self) -> GameValue {
        self.calculate_value_2()
    }

    pub fn calculate_value_1(&self) -> GameValue {
        let mut card_counts: HashMap<char, u32> = HashMap::new();

        self.cards.iter().for_each(|x| match card_counts.get(x) {
            Some(vc) => {
                card_counts.insert(*x, vc + 1);
            }
            None => {
                card_counts.insert(*x, 1);
            }
        });

        let mut count_value: Vec<_> = card_counts.values().collect();
        count_value.sort();
        count_value.reverse();

        let max_count = **count_value.first().unwrap();
        let second_max_count = match count_value.get(1) {
            Some(t) => **t,
            None => 0,
        };

        match max_count {
            5 => GameValue::FiveOfAKind,
            4 => GameValue::FourOfAKind,
            3 if second_max_count == 2 => GameValue::FullHouse,
            3 => GameValue::ThreeOfAKind,
            2 if second_max_count == 2 => GameValue::TwoPair,
            2 => GameValue::OnePair,
            _ => GameValue::HighCard,
        }
    }
    pub fn calculate_value_2(&self) -> GameValue {
        let cards: Vec<_> = self
            .cards
            .iter()
            .filter(|x| **x != 'J')
            .copied()
            .collect();
        let n_jokers = self.cards.iter().filter(|x| **x == 'J').count();

        let mut card_counts: HashMap<char, u32> = HashMap::new();

        cards.iter().for_each(|x| match card_counts.get(x) {
            Some(vc) => {
                card_counts.insert(*x, vc + 1);
            }
            None => {
                card_counts.insert(*x, 1);
            }
        });

        let mut count_value: Vec<_> = card_counts.values().collect();
        count_value.sort();
        count_value.reverse();

        let max_count = match count_value.first() {
            Some(t) => **t + TryInto::<u32>::try_into(n_jokers).unwrap(),
            None => TryInto::<u32>::try_into(n_jokers).unwrap(),
        };

        let second_max_count = match count_value.get(1) {
            Some(t) => **t,
            None => 0,
        };

        match max_count {
            5 => GameValue::FiveOfAKind,
            4 => GameValue::FourOfAKind,
            3 if second_max_count >= 2 => GameValue::FullHouse,
            3 => GameValue::ThreeOfAKind,
            2 if second_max_count >= 2 => GameValue::TwoPair,
            2 => GameValue::OnePair,
            1 => GameValue::HighCard,
            _ => {
                panic!("Error!")
            }
        }
    }

    fn order_1(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_value_1().eq(&other.get_value_1()) {
            false => {
                return Some(self.get_value_1().cmp(&other.get_value_1()));
            }
            true => {
                for i in 0..self.cards.len() {
                    let cs = self.cards.get(i).unwrap();
                    let co = other.cards.get(i).unwrap();

                    let csv = card_value_part1(cs);
                    let cov = card_value_part1(co);

                    match csv.cmp(&cov) {
                        Ordering::Equal => {
                            continue;
                        }
                        Ordering::Greater => {
                            return Some(Ordering::Greater);
                        }
                        Ordering::Less => {
                            return Some(Ordering::Less);
                        }
                    }
                }
            }
        };

        None
    }

    fn order_2(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_value_2().eq(&other.get_value_2()) {
            false => {
                return Some(self.get_value_2().cmp(&other.get_value_2()));
            }
            true => {
                for i in 0..self.cards.len() {
                    let cs = self.cards.get(i).unwrap();
                    let co = other.cards.get(i).unwrap();

                    let csv = card_value_part2(cs);
                    let cov = card_value_part2(co);

                    match csv.cmp(&cov) {
                        Ordering::Equal => {
                            continue;
                        }
                        Ordering::Greater => {
                            return Some(Ordering::Greater);
                        }
                        Ordering::Less => {
                            return Some(Ordering::Less);
                        }
                    }
                }
            }
        };
        None
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn card_value_part1(c: &char) -> u8 {
    "AKQJT98765432"
        .chars()
        .rev()
        .position(|r| r == *c)
        .unwrap()
        .try_into()
        .unwrap()
}

fn card_value_part2(c: &char) -> u8 {
    "AKQT98765432J"
        .chars()
        .rev()
        .position(|r| r == *c)
        .unwrap()
        .try_into()
        .unwrap()
}

pub fn day_7_1() {
    let input: Vec<&str> = include_str!("../input/day_7_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut games: Vec<Game> = input
        .iter()
        .map(|l| {
            let (cards, bid_str) = l.split_once(' ').unwrap();

            let mut g = Game::new(bid_str.parse().unwrap());
            cards.chars().for_each(|c| {
                g.add_card(c);
            });
            g
        })
        .collect();

    games.sort_by(|a, b| a.order_1(b).unwrap());

    let total_winnings_part1 = games
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, g)| acc + (idx + 1) * (g.bid as usize));

    println!("Day 7 Part 1 answer: {}", total_winnings_part1);

    games.sort_by(|a, b| a.order_2(b).unwrap());

    let total_winnings_part2 = games
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, g)| acc + (idx + 1) * (g.bid as usize));

    println!("Day 7 Part 2 answer: {}", total_winnings_part2);

    // 248569531
    // 250382098
}
