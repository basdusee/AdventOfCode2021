use std::fs::File;
use std::io::prelude::*;

pub fn readinput(filename: &str) -> Vec<usize> {
    
    // new and improved way to load input files, learned last year.
    // Don't know if it is faster, but it is waaay shorter and more comprehensible
    let mut input = String::new();
    let mut file = File::open(filename).expect("Could not read input.txt");
    file.read_to_string(&mut input).expect("Could not read input.txt");


    // let's parse all the numbers into a vector and return
    input.trim().split('\n')
                .map(|x| x.parse::<usize>().unwrap_or_default()) 
                .collect()

}
