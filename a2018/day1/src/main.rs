use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;


fn main() {
    let mut total = 0;

    // Create a path to the desired file
    let path = Path::new("./day1.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    let _err = file.read_to_string(&mut s);

    let mut array = Vec::new();
    for line in s.split("\n") {
        if line == "" {
            break;
        }
        let my_int: i32 = line.parse::<i32>().unwrap();
        array.push(my_int);
    }


    let mut past_results = HashSet::new();
     for number in array.iter().cycle() {
         total += number;
         if past_results.contains(&total) {
            println!("first duplicate found {}", total);
            break;
        }
        past_results.insert(total);
     }

    println!("total: {}", total);
}

