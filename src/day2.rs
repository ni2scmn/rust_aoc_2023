#[derive(Debug)]
struct Game {
    green: i32,
    blue: i32,
    red: i32,
}

impl Game {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    pub fn new() -> Game {
        Game {
            blue: 0,
            green: 0,
            red: 0,
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.blue > Game::MAX_BLUE ||
            self.green > Game::MAX_GREEN ||
            self.red > Game::MAX_RED {
            return false;
        }
        true
    }
}

pub fn day_2_1() {
    let input: Vec<&str> = include_str!("../input/day_2_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut valid_id_sum: i32 = 0;

    for l in input {
        let (game_id_str, draws_str) = l.split_once(':').unwrap();
        let game_id = game_id_str.replace("Game ", "").parse::<i32>().unwrap();
        let mut game_is_valid = true;

        draws_str.split(';').for_each(|d_str| {
            let mut game = Game::new();

            d_str.split(',').for_each(|c_str| {
                let (_, c_str) = c_str.split_once(' ').unwrap();
                let (q_str, c) = c_str.split_once(' ').unwrap();

                let q = q_str.parse::<i32>().unwrap();

                match c {
                    "red" => game.red = q,
                    "green" => game.green = q,
                    "blue" => game.blue = q,
                    _ => {
                        panic!("Unexpected")
                    }
                }
            });

            if !game.is_valid() {
                game_is_valid = false
            }
        });

        if game_is_valid {
            valid_id_sum += game_id;
        }
    }

    println!("Day 2 Part 1 answer: {}", valid_id_sum);
}
