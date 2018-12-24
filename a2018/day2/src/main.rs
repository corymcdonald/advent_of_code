const PUZZLE: &str = include_str!("day2.txt");

fn count(line: &str) -> (i32, i32) {
    let mut two = 0;
    let mut three = 0;
    for x in line.chars() {
        let count = line.matches(x).count();
        if count == 2 { two = 1; }
        if count == 3 { three = 1; }
    }

    return (two, three);
}

fn main() {
    let mut count_of_two = 0;
    let mut count_of_three = 0;

    PUZZLE
        .lines()
        .for_each( |n| {
            let (two,three) = count(n);
            count_of_two += two;
            count_of_three += three;
        });

    println!("result {}", count_of_two * count_of_three);
}
