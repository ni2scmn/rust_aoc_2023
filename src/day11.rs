use std::cmp;

#[derive(Debug)]
struct GalaxyPos(usize, usize);

impl GalaxyPos {
    pub fn distance_to(
        &self,
        other: &GalaxyPos,
        empty_rows: &Vec<usize>,
        empty_cols: &Vec<usize>,
        expansion_factor: isize,
    ) -> isize {
        let rmin = cmp::min(self.0 as isize, other.0 as isize);
        let rmax = cmp::max(self.0 as isize, other.0 as isize);
        let cmin = cmp::min(self.1 as isize, other.1 as isize);
        let cmax = cmp::max(self.1 as isize, other.1 as isize);

        let mut rdist = rmax - rmin;
        let mut cdist = cmax - cmin;

        for er in empty_rows.iter() {
            if *er > rmin as usize && *er < rmax as usize {
                rdist += expansion_factor;
            }
        }

        for ec in empty_cols.iter() {
            if *ec > cmin as usize && *ec < cmax as usize {
                cdist += expansion_factor;
            }
        }

        rdist + cdist
    }
}

pub fn day_11_1() {
    let input: Vec<&str> = include_str!("../input/day_11_1_example.txt")
        .split('\n')
        .collect::<Vec<_>>();

    let mut map: Vec<Vec<char>> = Vec::new();

    let mut galaxies: Vec<GalaxyPos> = Vec::new();

    for l in input.iter() {
        map.push(l.chars().collect());
    }

    let n_rows = input.len();
    let n_cols = input.first().unwrap().len();

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    // find empty rows
    for (r_idx, l) in map.iter().enumerate() {
        let mut empty = true;

        for f in l {
            if *f == '#' {
                empty = false;
            }
        }
        if empty {
            empty_rows.push(r_idx);
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
            empty_cols.push(c_idx);
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

    let mut part1_total_distance: isize = 0;
    let mut part2_total_distance: isize = 0;

    for g1_idx in 0..galaxies.len() {
        for g2_idx in g1_idx..galaxies.len() {
            if g1_idx == g2_idx {
                continue;
            }
            part1_total_distance += galaxies.get(g1_idx).unwrap().distance_to(
                galaxies.get(g2_idx).unwrap(),
                &empty_rows,
                &empty_cols,
                1,
            );
            part2_total_distance += galaxies.get(g1_idx).unwrap().distance_to(
                galaxies.get(g2_idx).unwrap(),
                &empty_rows,
                &empty_cols,
                1000000 - 1,
            );
        }
    }

    println!("Day 11 Part 1 answer: {}", part1_total_distance);
    println!("Day 11 Part 2 answer: {}", part2_total_distance);
}
