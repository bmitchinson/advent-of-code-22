// use std::env;
use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("Reading file: {file_path}");

    // refactor: instead of splitting, read as vector / iterable
    // note: ensure it skips final line instead of accounting for it in parse
    let file_contents =
        fs::read_to_string(file_path).expect("Should have read in the file {file_path}");

    let depths = file_contents.split("\n");
    // note: set this to initialize at "None" for readability?
    let mut prev_depth = 0;
    let mut increases = -1; // account for initial depth

    // note: is there a way to convert in place? Instead of depth_int reassignment?
    for depth in depths {
        match depth.parse::<i32>() {
            Ok(depth_int) => {
                if depth_int > prev_depth {
                    increases += 1;
                }
                prev_depth = depth_int;
            }
            _ => (),
        }
    }

    println!("Depth increases: {increases}");
}
