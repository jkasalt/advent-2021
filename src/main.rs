use advent_2021::*;
use clap::Parser;
use std::borrow::Borrow;
use std::fmt::Display;
use std::fs;
use std::time::Instant;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    day: u8,

    #[clap(short, long)]
    part: Option<u8>,
}
fn run_with_two_gen<G, D, K: ?Sized>(
    p1: impl Fn(&K) -> D,
    p2: Option<impl Fn(&K) -> D>,
    gen1: impl Fn(&str) -> G,
    gen2: Option<impl Fn(&str) -> G>,
    input_path: &str,
) where
    G: Borrow<K>,
    D: Display,
{
    let input = &fs::read_to_string(input_path).expect("correct path");
    let built1 = gen1(input);
    println!("{}", p1(built1.borrow()));
    if let Some(p2) = p2 {
        let built2 = gen2.unwrap()(input);
        println!("{}", p2(built2.borrow()));
    }
}

fn run_with_gen<G, D, K: ?Sized>(
    p1: impl Fn(&K) -> D,
    p2: Option<impl Fn(&K) -> D>,
    gen: impl Fn(&str) -> G,
    input_path: &str,
) where
    G: Borrow<K>,
    D: Display,
{
    let input = &fs::read_to_string(input_path).expect("correct path");
    let built = gen(input);
    println!("{}", p1(built.borrow()));
    if let Some(p2) = p2 {
        println!("{}", p2(built.borrow()));
    }
}

fn run<D: Display>(p1: impl Fn(&str) -> D, p2: Option<impl Fn(&str) -> D>, input_path: &str) {
    let input = &fs::read_to_string(input_path).expect("correct path");
    println!("{}", p1(input));
    if let Some(p2) = p2 {
        println!("{}", p2(input));
    }
}

fn main() {
    let args = Args::parse();
    let i_path = &format!("input/2021/day{}.txt", args.day);
    let time_start = Instant::now();

    match args.day {
        1 => {
            run_with_gen(day1::first, Some(day1::second), day1::gen, i_path);
        }
        2 => {
            run(day2::first, Some(day2::second), i_path);
        }
        3 => {
            run_with_gen(day3::first, Some(day3::second), day3::gen, i_path);
        }
        4 => {
            run_with_gen(day4::first, Some(day4::second), day4::gen, i_path);
        }
        5 => {
            run(day5::first, Some(day5::second), i_path);
        }
        6 => {
            run(day6::first, Some(day6::second), i_path);
        }
        7 => {
            run_with_gen(day7::first, Some(day7::second), day7::gen, i_path);
        }
        8 => {
            run(day8::first, Some(day8::second), i_path);
        }
        9 => {
            run_with_gen(day9::first, Some(day9::second), day9::gen, i_path);
        }
        10 => {
            run(day10::first, Some(day10::second), i_path);
        }
        11 => {
            run_with_gen(day11::first, Some(day11::second), day11::gen, i_path);
        }
        12 => {
            run_with_gen(day12::first, Some(day12::second), day12::gen, i_path);
        }
        13 => {
            run_with_gen(day13::first, Some(day13::second), day13::gen, i_path);
        }
        14 => {
            run_with_gen(day14::first, Some(day14::second), day14::gen, i_path);
        }
        15 => {
            run_with_two_gen(
                day15::first,
                Some(day15::second),
                day15::gen,
                Some(day15::gen2),
                i_path,
            );
        }
        16 => {
            run_with_gen(day16::first, Some(day16::second), day16::hex_to_bin, i_path);
        }
        18 => {
            run(day18::first, Some(day18::second), i_path);
        }
        20 => {
            run(day20::first, Some(day20::second), i_path);
        }
        21 => {
            run(day21::first, Some(day21::second), i_path);
        }
        _ => {}
    }

    let time_end = Instant::now();
    println!("total time: {:?}", time_end.duration_since(time_start));
}
