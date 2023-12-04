pub fn day4_1() {
    let input: Vec<&str> = include_str!("../input/day_4_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut winning_sum = 0;

    for l in input {
        let (_game_id, game) = l.split_once(':').unwrap();

        let (winning_cards_str, drawn_cards_str) = game.split_once('|').unwrap();

        let winning_cards: Vec<_> = winning_cards_str
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>())
            .filter_map(|x| x.ok())
            .collect();

        let drawn_cards: Vec<_> = drawn_cards_str
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>())
            .filter_map(|x| x.ok())
            .collect();

        let mut round_sum = 0;

        winning_cards.iter().for_each(|x| {
            for dc in drawn_cards.iter() {
                if *x == *dc {
                    if round_sum == 0 {
                        round_sum = 1;
                    } else {
                        round_sum *= 2;
                    }
                    break;
                }
            }
        });
        winning_sum += round_sum;
    }

    println!("Day 4 Part 1 answer: {}", winning_sum);
}
