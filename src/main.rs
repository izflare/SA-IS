extern crate sais;
extern crate clap;

use clap::{App, Arg};
use std::io::{prelude::*, BufReader};
use std::fs::File;
use std::time::Instant;
use sais::suffixarray;

fn main() {
    let app = App::new("sais")
        //{{{
        .version("0.1.0")                       
        .author("flare")     
        .about("linear time suffix array constructor")
        .arg(Arg::with_name("input")
            .help("filename of input sourse text")
            .short("i")
            .long("input")
            .takes_value(true)
            .required(true)
        );
        //}}}
    let matches = app.get_matches();

    let mut s = String::new();
    let mut f = BufReader::new(File::open(&matches.value_of("input").unwrap()).expect("file not found"));
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
