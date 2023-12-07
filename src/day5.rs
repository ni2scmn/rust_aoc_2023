fn parse_seed(l: &str) -> Vec<i64> {
    l.replace("seeds: ", "")
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub fn day5_1() {
    let input: Vec<&str> = include_str!("../input/day_5_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    let mut cur_map: Option<Vec<(i64, i64, i64)>> = None;

    let mut seeds: Option<Vec<i64>> = None;
    for l in input {
        // println!("inputline: {}", l);

        if l.contains("seeds") {
            seeds = Some(parse_seed(l));
        } else if l.contains("map") {
            if let Some(cm) = cur_map {
                maps.push(cm);
            }
            cur_map = Some(Vec::new());
        } else if l.chars().count() == 0 {
            continue;
        } else {
            let map_rule = l
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let destination_start = map_rule.first().unwrap();
            let source_start = map_rule.get(1).unwrap();
            let range_length = map_rule.get(2).unwrap();

            // println!("destination_start {}", destination_start);
            // println!("source_start {}", source_start);
            // println!("range_length {}", range_length);

            let shift = destination_start - source_start;
            let source_end = source_start + range_length;

            cur_map
                .as_mut()
                .unwrap()
                .push((*source_start, source_end, shift));

            // println!("maprule: {:?}", map_rule);
            // println!("current map rules{:?}", cur_map.as_ref().unwrap());
        }
    }
    if let Some(cm) = cur_map {
        maps.push(cm);
    }

    // println!("seeds {:?}", seeds.as_ref().unwrap());

    let mut minimum = i64::MAX;

    for s in seeds.as_ref().unwrap() {
        let mut seed = *s;

        for (_idx, m) in maps.iter().enumerate() {
            // println!("map: {:?}", m);

            // println!("pre_map {}: {}", idx, seed);

            for mr in m {
                if seed >= mr.0 && seed < mr.1 {
                    seed += mr.2;
                    break;
                }
            }

            // println!("post_map {}: {}", idx, seed);
        }

        if seed < minimum {
            minimum = seed;
        }
    }

    println!("Day 5 Part 1 answer: {}", minimum);
}
