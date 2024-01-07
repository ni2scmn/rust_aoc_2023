#[derive(Debug)]
struct GalaxyPos(usize, usize);

impl GalaxyPos {
    pub fn distance_to(&self, other: &GalaxyPos) -> isize {
        let rdist = (self.0 as isize - other.0 as isize).abs();
        let cdist = (self.1 as isize - other.1 as isize).abs();
        rdist + cdist
    } 
}


pub fn day_11_1() {
    let input: Vec<&str> = include_str!("../input/day_11_1_input.txt")
       .split('\n')
        .collect::<Vec<_>>();

    let mut map: Vec<Vec<char>> = Vec::new();

    let mut galaxies: Vec<GalaxyPos> = Vec::new();

    for l in input.iter() {
        map.push(l.chars().collect());
    }

    let n_rows = input.len();
    let n_cols = input.first().unwrap().len();

    let mut row_dupl: Vec<usize> = Vec::new();
    let mut col_dupl: Vec<usize> = Vec::new();


    // find empty rows
    for (r_idx, l) in map.iter().enumerate() {
        let mut empty = true;
        
        for f in l {
            if *f == '#'{
                empty = false;
            } 
        }
        if empty {
            // println!("row: {} is empty", r_idx);
            row_dupl.push(r_idx);
        }
    }

    // find empty columns
    for c_idx in 0..n_cols {
        let mut empty = true;

        for l in map.iter() {
            if l[c_idx] == '#' {
                empty = false;
            }
        }
        if empty {
            // println!("column: {} is empty", c_idx);
            col_dupl.push(c_idx);
        }
    }


    // duplicate empty rows
    while !row_dupl.is_empty() {
        let row_to_dupl = row_dupl.pop().unwrap();
        let dupl = vec!['.'; n_cols];
        map.insert(row_to_dupl, dupl);

        for r in row_dupl.iter_mut() {
            *r += 1;
        }
    }

    // duplicate empty cols
    while !col_dupl.is_empty() {
        let col_to_dupl = col_dupl.pop().unwrap();

        for r in map.iter_mut() {
            r.insert(col_to_dupl, '.');
        }
    }

    // find galaxies
    for (r_idx, r) in map.iter().enumerate() {
        for (c_idx, f) in r.iter().enumerate() {
            if *f == '#' {
                galaxies.push(GalaxyPos(r_idx, c_idx));
            }
        }
    }


    // print map
    // for r in map.iter() {
    //     for f in r.iter() {
    //         print!("{}", f);
    //     }
    //     println!("");
    // }

    // print galaxies
    // for g in galaxies.iter() {
    //     println!("{:?}", g);
    // }

    let mut total_distance: isize = 0;

    for g1_idx in 0..galaxies.len() {
        for g2_idx in g1_idx..galaxies.len() {
            if g1_idx == g2_idx {
                continue;
            }
            total_distance += galaxies.get(g1_idx).unwrap().distance_to(galaxies.get(g2_idx).unwrap());
        }
    }

    println!("Day 11 Part 1 answer: {}", total_distance);
}
   
