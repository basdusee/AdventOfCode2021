use std::fs::File;
use std::io::prelude::*;

pub fn readinput(filename: &str) -> Vec<usize> {
    
    let mut input = String::new();
    let mut file = File::open(filename).expect("Could not read input.txt");
    file.read_to_string(&mut input).expect("Could not read input.txt");


    // let's parse all the numbers into a vector and return
    input.trim().split('\n')
                .map(|x| x.parse::<usize>().unwrap_or_default()) 
                .collect()

}
