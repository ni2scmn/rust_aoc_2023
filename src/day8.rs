use core::panic;
use std::collections::HashMap;
use regex::Regex;

pub fn day_8_1() {

    let mut ways: HashMap<String, (String, String)> = HashMap::new();

    let input: Vec<&str> = include_str!("../input/day_8_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let instructions = input.first().unwrap();
    let instruction_size = instructions.chars().count();

    for l in input.iter().skip(2) {
        let (start, alternatives) = l.split_once(" = ").unwrap();
        let alternatives2 = alternatives.replace("(", "").replace(")", "");

        let (al_1, al_2) = alternatives2.split_once(", ").unwrap();
        ways.insert(start.into(), (al_1.into(), al_2.into()));
    }

    let mut position: String = "AAA".into();

    let mut n_jumps = 0;

    while position != "ZZZ" {

        let cur_instruction = instructions.chars().nth(n_jumps % instruction_size).unwrap();

        println!("cur_pos: {}, cur_instr: {}, cur_alt: {:?}", position, cur_instruction, ways.get(&position).unwrap());

        match cur_instruction {
            'R' => {
                position = ways.get(&position).unwrap().1.clone();
            },
            'L' => {
                position = ways.get(&position).unwrap().0.clone(); 
            }
            _ => {panic!("OOOPS")}
        }

        n_jumps += 1;

        if n_jumps > 12 {
            // break;
        }
    }


    println!("{}", n_jumps);
}
