#![allow(dead_code)]
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX_SCA: Lazy<Regex> = Lazy::new(|| Regex::new(r"--- scanner (\d+)").unwrap());
static REGEX_POS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap());

struct Scanner {
    num: u32,
    detected: Vec<[i32; 3]>,
}

pub fn first(_input: &str) -> u32 {
    // let mut scanners: Vec<[i32; 3]> = Vec::new();
    // let cap = REGEX_SCA
    //     .captures(input.lines().next().unwrap())
    //     .unwrap()
    //     .get(1)
    //     .expect("valid input");
    // for line in input.lines() {
    //     if REGEX_SCA.is_match(line) {}
    // }
    todo!()
}
