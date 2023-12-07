use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Game {
    cards: Vec<char>,
    value: Option<GameValue>,
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
    FiveOfAKind
}

impl Game {
    pub fn new(bid: u32) -> Game {
        Game {
            cards: Vec::new(),
            value: None,
            bid: bid,
        }
    }

    pub fn add_card(&mut self, c: char) {
        self.cards.push(c);
        self.value = None;
    }

    pub fn get_value(&mut self) -> GameValue {
        if self.value.is_none() {
            self.value = Some(self.calculate_value());
        }
        return self.value.unwrap();
    }

    pub fn get_value_(&self) -> GameValue {
        if self.value.is_none() {
            return self.calculate_value();
        }
        return self.value.unwrap();
    }

    pub fn calculate_value(&self) -> GameValue {
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
        // let second_max_count = **count_value.get(1).unwrap();

        let second_max_count = match count_value.get(1) {
            Some(t) => **t,
            None => 0
        };

        let game_value = match max_count {
            5 => GameValue::FiveOfAKind,
            4 => GameValue::FourOfAKind,
            3 if second_max_count == 2 => GameValue::FullHouse,
            3 => GameValue::ThreeOfAKind,
            2 if second_max_count == 2 => GameValue::TwoPair,
            2 => GameValue::OnePair,
            _ => GameValue::HighCard,
        };
        game_value
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_value_().eq(&other.get_value_()) {
            false => {
                return Some(self.get_value_().cmp(&other.get_value_()));
            }
            true => {
                for i in 0..self.cards.len() {
                    let cs = self.cards.get(i).unwrap();
                    let co = other.cards.get(i).unwrap();

                    let csv = card_value(cs);
                    let cov = card_value(co);

                    if csv == cov {
                        continue;
                    } else if csv > cov {
                        return Some(Ordering::Greater);
                    } else {
                        return Some(Ordering::Less);
                    }
                }
            }
        };

        None
    }
}

fn card_value(c: &char) -> u8 {
    "AKQJT98765432"
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

    games.iter_mut().for_each(|x| {
        x.get_value();
    });


    games.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let x = games.iter().enumerate().fold(0, |acc, (idx, g) | {
        acc + (idx + 1) * (g.bid as usize)
    });

    println!("{}", x);

    // 248569531
}
