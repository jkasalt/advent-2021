use advent_2021::*;
use clap::Parser;
use std::borrow::Borrow;
use std::fmt::Display;
use std::fs;
use std::thread;
use std::time::Instant;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    day: Option<u8>,

    #[clap(short, long)]
    part: Option<u8>,
}
fn run_with_two_gen<G, D, K: ?Sized>(
    p1: impl Fn(&K) -> D,
    p2: impl Fn(&K) -> D,
    gen1: impl Fn(&str) -> G,
    gen2: impl Fn(&str) -> G,
    input_path: &str,
) where
    G: Borrow<K>,
    D: Display,
{
    let input = &fs::read_to_string(input_path).expect("correct path");
    let built1 = gen1(input);
    println!("first: {}", p1(built1.borrow()));
    let built2 = gen2(input);
    println!("second: {}", p2(built2.borrow()));
}

fn run_with_gen<G, D, K: ?Sized>(
    p1: impl Fn(&K) -> D,
    p2: impl Fn(&K) -> D,
    gen: impl Fn(&str) -> G,
    input_path: &str,
) where
    G: Borrow<K>,
    D: Display,
{
    let input = &fs::read_to_string(input_path).expect("correct path");
    let built = gen(input);
    println!("first: {}", p1(built.borrow()));
    println!("second: {}", p2(built.borrow()));
}

fn run<D: Display, F>(p1: F, p2: impl Fn(&str) -> D, input_path: &str)
where
    F: Send + 'static + Sync + Fn(&str) -> D,
{
    let input = fs::read_to_string(input_path).expect("correct path");
    let input_clone = input.clone();
    let first_problem = thread::spawn(move || println!("first: {}", p1(&input_clone)));
    println!("second: {}", p2(&input));
    first_problem.join().unwrap();
}

fn main() {
    let args = Args::parse();
    let range = match args.day {
        None => 1..=25,
        Some(d) => d..=d,
    };

    for d in range {
        println!("\n day: {}", d);
        let time_start = Instant::now();
        let i_path = &format!("input/2021/day{}.txt", d);
        match d {
            1 => {
                run_with_gen(day1::first, day1::second, day1::gen, i_path);
            }
            2 => {
                run(day2::first, day2::second, i_path);
            }
            3 => {
                run_with_gen(day3::first, day3::second, day3::gen, i_path);
            }
            4 => {
                run_with_gen(day4::first, day4::second, day4::gen, i_path);
            }
            5 => {
                run(day5::first, day5::second, i_path);
            }
            6 => {
                run(day6::first, day6::second, i_path);
            }
            7 => {
                run_with_gen(day7::first, day7::second, day7::gen, i_path);
            }
            8 => {
                run(day8::first, day8::second, i_path);
            }
            9 => {
                run_with_gen(day9::first, day9::second, day9::gen, i_path);
            }
            10 => {
                run(day10::first, day10::second, i_path);
            }
            11 => {
                run_with_gen(day11::first, day11::second, day11::gen, i_path);
            }
            12 => {
                run_with_gen(day12::first, day12::second, day12::gen, i_path);
            }
            13 => {
                run_with_gen(day13::first, day13::second, day13::gen, i_path);
            }
            14 => {
                run_with_gen(day14::first, day14::second, day14::gen, i_path);
            }
            15 => {
                run_with_two_gen(day15::first, day15::second, day15::gen, day15::gen2, i_path);
            }
            16 => {
                run_with_gen(day16::first, day16::second, day16::hex_to_bin, i_path);
            }
            18 => {
                run(day18::first, day18::second, i_path);
            }
            20 => {
                run(day20::first, day20::second, i_path);
            }
            21 => {
                run(day21::first, day21::second, i_path);
            }
            22 => run(day22::first, day22::second, i_path),
            23 => run_with_two_gen(day23::first, day23::second, day23::parse, day23::parse_second, i_path),
            _ => {}
        }
        let time_end = Instant::now();
        println!("total time: {:?}", time_end.duration_since(time_start));
    }
}
