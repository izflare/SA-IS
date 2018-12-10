extern crate sais;

use std::env;
use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::time::Instant;
use sais::suffixarray;

fn usage(s: &str) -> () {
    println!("Usage: {} <filename>", s);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if (&args).len() != 2 {
        usage(&args[0]);
        return;
    }
    let mut s = String::new();
    let mut f = BufReader::new(File::open(&args[1]).expect("file not found"));
    f.read_to_string(&mut s).unwrap();

    let mut v: Vec<usize> = s.chars().map(|c| c as usize).collect();
    v.push(0);
    let mut sa: Vec<Option<usize>> = vec![None; v.len()];
    let start = Instant::now();
    suffixarray::create(&v, &mut sa);
    let end = start.elapsed();

    println!("{}.{:09} sec elapsed", end.as_secs(), end.subsec_nanos());
    println!("Suffix Array is: ");
    println!("{:?}", sa.iter().skip(1).map(|x| x.unwrap()).collect::<Vec<usize>>());
}
