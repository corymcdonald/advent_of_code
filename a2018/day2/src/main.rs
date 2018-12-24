
const PUZZLE: &str = include_str!("day2.txt");

// fn count(line: &str) -> (i32, i32) {
//     let mut two = 0;
//     let mut three = 0;
//     for x in line.chars() {
//         let count = line.matches(x).count();
//         if count == 2 { two = 1; }
//         if count == 3 { three = 1; }
//     }

//     return (two, three);
// }


fn difference(a : Vec<char>, b: Vec<char>) -> (Vec<String>, i32) {
    let mut diff = 0;
    let mut similarities = Vec::new();
    for i in 0..a.len() {
        if a[i] == b[i] {
            similarities.push(a[i].to_string());
        } else {
            diff += 1;
        }
    }

    return (similarities, diff);
}

fn part2() {
    // let mut array_of_sets = Vec::new();

    // let (similarities, diff) = difference(a, b);



    // // Create list of hashes so we can find the set difference
    PUZZLE
        .lines()
        .for_each( |x| {
            PUZZLE.lines().for_each( |y| {

               let (similarities, diff) = difference(x.chars().collect(), y.chars().collect());
               if diff == 1 {
                   for s in similarities {
                       print!("{}", s);
                   }
                   println!("");
               }


            })
        });


}


fn main() {
    // let mut count_of_two = 0;
    // let mut count_of_three = 0;

    // PUZZLE
    //     .lines()
    //     .for_each( |n| {
    //         let (two,three) = count(n);
    //         count_of_two += two;
    //         count_of_three += three;
    //     });

    part2();
    // println!("result {}", count_of_two * count_of_three);
}
