use std::collections::HashMap;

pub fn greets() {
    println!("Hello, world!");
}

fn calculate_sum(input: &Vec<&str>) -> u32 {
    let mut digit_sum: u32 = 0;

    for x in input {
        let mut first_digit: Option<u32> = None;

        for f_cand in x.chars() {
            let digit = char::to_digit(f_cand, 10);

            if let Some(d) = digit {
                first_digit = Some(d);
                break;
            }
        }

        let mut last_digit: Option<u32> = None;

        for l_cand in x.chars().rev() {
            let digit = char::to_digit(l_cand, 10);
            if let Some(d) = digit {
                last_digit = Some(d);
                break;
            }
        }

        digit_sum += first_digit.unwrap() * 10;
        digit_sum += last_digit.unwrap();
    }

    digit_sum
}

pub fn day_1_1() {
    let input: Vec<&str> = include_str!("../input/day_1_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    println!("Day 1 Part 1 answer: {}", calculate_sum(&input));
}

pub fn day_1_2() {
    let input = include_str!("../input/day_1_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    let mut digit_sum = 0;

    for x in input.iter() {
        let mut first_digit_space: String = "".to_owned();
        let mut first_digit: Option<u32> = None;

        let mut last_digit_space: String = "".to_owned();
        let mut last_digit: Option<u32> = None;

        'outer: for xd in x.chars() {
            for (dd, ds) in digits.iter() {
                if first_digit_space.contains(dd) {
                    first_digit = Some(*ds);
                    break 'outer;
                }
            }

            let digit = char::to_digit(xd, 10);
            if let Some(d) = digit {
                first_digit = Some(d);
                break;
            }

            first_digit_space.push(xd);
        }

        'outer: for xd in x.chars().rev() {
            for (dd, ds) in digits.iter() {
                let dd_rev = dd.chars().rev().collect::<String>();

                if last_digit_space.contains(&dd_rev) {
                    last_digit = Some(*ds);
                    break 'outer;
                }
            }

            let digit = char::to_digit(xd, 10);
            if let Some(d) = digit {
                last_digit = Some(d);
                break;
            }

            last_digit_space.push(xd);
        }

        digit_sum += first_digit.unwrap() * 10;
        digit_sum += last_digit.unwrap();
    }
    println!("Day 1 Part 2 answer: {}", digit_sum);
}
