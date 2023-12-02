#[derive(Debug)]
struct CubeSet {
    green: i32,
    blue: i32,
    red: i32,
}

impl CubeSet {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    pub fn is_valid(&self) -> bool {
        if self.blue > CubeSet::MAX_BLUE
            || self.green > CubeSet::MAX_GREEN
            || self.red > CubeSet::MAX_RED
        {
            return false;
        }
        true
    }

    pub fn cube_power(&self) -> i32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct Game {
    cubes: CubeSet,
}

impl Game {
    pub fn new() -> Game {
        Game {
            cubes: CubeSet {
                green: 0,
                red: 0,
                blue: 0,
            },
        }
    }
}

struct GameSet {
    games: Vec<Game>,
}

impl GameSet {
    pub fn new() -> GameSet {
        GameSet { games: Vec::new() }
    }

    pub fn is_valid(&self) -> bool {
        self.games
            .iter()
            .all(|x| x.cubes.is_valid())
    }

    pub fn minimal_game(&self) -> Game {
        let mut min_game = Game::new();

        self.games.iter().for_each(|g| {
            if g.cubes.red > min_game.cubes.red {
                min_game.cubes.red = g.cubes.red
            }
            if g.cubes.blue > min_game.cubes.blue {
                min_game.cubes.blue = g.cubes.blue
            }
            if g.cubes.green > min_game.cubes.green {
                min_game.cubes.green = g.cubes.green
            }
        });

        min_game
    }
}

pub fn day_2_1() {
    let input: Vec<&str> = include_str!("../input/day_2_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut valid_id_sum: i32 = 0;
    let mut min_cube_power_sum: i32 = 0;

    for l in input {
        let (game_id_str, draws_str) = l.split_once(':').unwrap();
        let game_id = game_id_str.replace("Game ", "").parse::<i32>().unwrap();

        let mut game_set = GameSet::new();

        draws_str.split(';').for_each(|d_str| {
            let mut game = Game::new();

            d_str.split(',').for_each(|c_str| {
                let (_, c_str) = c_str.split_once(' ').unwrap();
                let (q_str, c) = c_str.split_once(' ').unwrap();

                let q = q_str.parse::<i32>().unwrap();

                match c {
                    "red" => game.cubes.red = q,
                    "green" => game.cubes.green = q,
                    "blue" => game.cubes.blue = q,
                    _ => {
                        panic!("Unexpected")
                    }
                }
            });

            game_set.games.push(game);
        });

        if game_set.is_valid() {
            valid_id_sum += game_id;
        }

        min_cube_power_sum += game_set.minimal_game().cubes.cube_power();
    }

    println!("Day 2 Part 1 answer: {}", valid_id_sum);
    println!("Day 2 Part 1 answer: {}", min_cube_power_sum);
}
