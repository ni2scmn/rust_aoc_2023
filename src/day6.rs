pub fn day_6_1() {

  let input: Vec<&str> = include_str!("../input/day_6_1_input.txt")
        .split('\n')
        .collect::<Vec<_>>();

  let times_str = input.first().unwrap().replace("Time: ", "");
  let times: Vec<i32> = times_str.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
  let distances_str = input.get(1).unwrap().replace("Distance: ", "");
  let distances: Vec<i32> = distances_str.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

  if times.len() != distances.len() {
      panic!("Unmatching times and distances size");
  }

  let mut winning_possibilities: Vec<i32> = Vec::new();

  for x in 0..times.len() {

      let mut round_winning_possibilities = 0;

      let t = times.get(x).unwrap();
      let d = distances.get(x).unwrap();

      println!("t: {}, d: {}", t, d);

      for y in 0..*t {
          let s = y;
          let dt = s * (*t - y);

          println!("s: {}, dt: {}", s, dt);

          if dt > *d {
              println!("Round won");
              round_winning_possibilities += 1;
          } else {
              println!("Round lost");
          }
      } 


      winning_possibilities.push(round_winning_possibilities);

  }

  println!("Day 6 Part 1 answer: {:?}", winning_possibilities.iter().product::<i32>());
}
