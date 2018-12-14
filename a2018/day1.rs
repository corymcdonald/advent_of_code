use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let mut total = 0;

    // Create a path to the desired file
    let path = Path::new("src/day1.txt");
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
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    for line in s.split("\n") {
        print!("{} is, ", line);
        let my_int: i32 = line.parse::<i32>().unwrap();
        print!("{}", my_int);
        total += my_int;
    }
    print!("{}", total);

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

