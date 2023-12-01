pub fn greets() {
    println!("Hello, world!");
}

pub fn day_1_1() {
    let input = include_str!("../input/day_1_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut digit_sum: u32 = 0;

    for x in input {

        println!("{}", x);

        let mut first_digit: Option<u32> = None;

        for f_cand in x.chars() {
            let digit = char::to_digit(f_cand, 10);
            match digit {
                Some(d) => {
                    first_digit = Some(d);
                    break;
                }
                _ => {}
            }
        }

        let mut last_digit: Option<u32> = None;

        for l_cand in x.chars().rev() {
            let digit = char::to_digit(l_cand, 10);
            match digit {
                Some(d) => {
                    last_digit = Some(d);
                    break;
                }
                _ => {}
            }
        }

        digit_sum +=  first_digit.unwrap() * 10;
        digit_sum +=  last_digit.unwrap();
        println!("{}", first_digit.unwrap());
        println!("{}", last_digit.unwrap());

    }

    println!("{}", digit_sum);
}