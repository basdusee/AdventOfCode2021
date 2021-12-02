use std::time::Instant;
use std::io::Error;
use itertools::Itertools;

use puzzle::*;


// This two functions are basically the solution.
// The whole main function is setup and result printing

fn increases(depths: &Vec<usize>) -> usize {
    depths.iter().tuple_windows().filter(|(a,b)| a < b).count()
}

fn three_measurement(depths: &Vec<usize>) -> usize {
    increases(&depths.iter().tuple_windows().map(|(a,b,c)| a + b + c).collect())
}


fn main() -> Result<(), Error> {
    
    // get the inputfile into memory
    let inputvec = readinput("input.txt"); 

    // start timing when the file is read in mem and we start to do stuff.
    let now = Instant::now();

    println!("Part One: larger measurements: {}", increases(&inputvec));

    println!("It took me: {} microseconds", now.elapsed().as_micros());

    // reset timing for part two.
    let now = Instant::now();
 
    println!("Part Two: three-measurement: {}", three_measurement(&inputvec));

    println!("It took me: {} microseconds", now.elapsed().as_micros());

    // Me main, me always happy
    Ok(())
}
