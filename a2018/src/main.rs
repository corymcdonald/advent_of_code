use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


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

    let mut vec: Vec<i32> = Vec::new();

    let mut is_duplicate = true;
    let mut index = 0;
    let mut t = 0;
    while is_duplicate {
        let current_int : i32 = array[index];

        total += current_int;

        // Part 2
        if vec.contains(&total) {
            println!("first duplicate found {}", total);
            is_duplicate = false;
        }
        vec.push(total);

        index += 1;
        if index >= array.len() {
            index = 0;
            t+=1;
            println!("size: {}", vec.len());
            println!("{}", t);
        }
    }

    // for item in vec {
    //     print!("{}, ", item);
    // }

    println!("total: {}", total);
}

