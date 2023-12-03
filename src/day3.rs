use std::cmp::{max, min};

#[derive(Debug, Default, Copy, Clone)]
struct Number {
    row: usize,
    col_start: usize,
    col_end: usize,
    value: u32,
    is_valid: bool
}

#[derive(Debug, Default)]
struct Map {
    nrow: usize,
    ncol: usize,
    values: Vec<Vec<char>>,
    number: Vec<Number>
}

fn check_surround(map: &Map, ridx: usize, cidx: usize) -> bool{
    for r in vec![ridx.checked_sub(1), Some(ridx), ridx.checked_add(1)] {

        if r.is_none() {
            continue;
        }

        let row_opt = map.values.get(r.unwrap());

        if let Some(row) = row_opt {
            for c in vec![cidx.checked_sub(1), Some(cidx), cidx.checked_add(1)] {

                if c.is_none() {
                    continue;
                }

                let cell_opt = row.get(c.unwrap());
                if let Some(cell) = cell_opt {
                    if !cell.is_digit(10) && *cell != '.' {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn day_3_1() {
    let input: Vec<&str> = include_str!("../input/day_3_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut map = Map::default();

    // build map
    input.iter().for_each(|l| {
        let mut colvec = Vec::new();

        l.chars().for_each(|c| {
            colvec.push(c);
        });
        map.values.push(colvec);
    });

    map.nrow = map.values.len();
    map.ncol = map.values.get(1).unwrap().len();

    let mut curr_number: Option<Number> = None;

    map.values.clone().iter().enumerate().for_each(|(ridx ,row) | {
        row.iter().enumerate().for_each(|(cidx, c)| {
            match (c.to_digit(10), &mut curr_number) {

                (Some(cd), Some(n)) => {
                    n.value = n.value * 10 + cd;

                    if !n.is_valid {
                        n.is_valid = check_surround(&map, ridx, cidx)
                    }

                },

                (Some(cd), None) => {
                    curr_number = Some(Number{
                        value: cd,
                        row: ridx,
                        col_start: cidx,
                        col_end: 0,
                        is_valid: check_surround(&map, ridx, cidx)
                    });
                },

                (None, Some(n)) => {
                    let col_end_op = cidx.checked_sub(1);
                    match col_end_op {
                        None => {n.col_end = map.ncol - 1}
                        Some(col_end) => {n.col_end = col_end}
                    }
                    map.number.push(*n);
                    curr_number = None;
                },

                (None, None) => {}
            }
        })
    });


    let sum = map.number.iter().fold(0, |acc, x| {
        if x.is_valid {
            return acc + x.value;
        } else {
            return acc;
        }
    });

    println!("Day 3 Part 1 answer: {}", sum);
}