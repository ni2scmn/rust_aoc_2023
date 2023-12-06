fn day_6_calc(ip: &str) -> i64 {
    let input: Vec<&str> = ip.split('\n').collect::<Vec<_>>();

    let times_str = input.first().unwrap().replace("Time: ", "");
    let times: Vec<i64> = times_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let distances_str = input.get(1).unwrap().replace("Distance: ", "");
    let distances: Vec<i64> = distances_str
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    if times.len() != distances.len() {
        panic!("Unmatching times and distances size");
    }

    let mut winning_possibilities: Vec<i64> = Vec::new();

    for x in 0..times.len() {
        let mut round_winning_possibilities = 0;

        let t = times.get(x).unwrap();
        let d = distances.get(x).unwrap();

        // println!("t: {}, d: {}", t, d);

        for y in 0..*t {
            let s = y;
            let dt = s * (*t - y);

            // println!("s: {}, dt: {}", s, dt);

            if dt > *d {
                // println!("Round won");
                round_winning_possibilities += 1;
            } else {
                // println!("Round lost");
            }
        }

        winning_possibilities.push(round_winning_possibilities);
    }

    return winning_possibilities.iter().product::<i64>();
}

pub fn day_6_1() {
    println!(
        "Day 6 Part 1 answer: {:?}",
        day_6_calc(include_str!("../input/day_6_1_input.txt"))
    );

    println!(
        "Day 6 Part 2 answer: {:?}",
        day_6_calc(include_str!("../input/day_6_2_input.txt"))
    );
}
