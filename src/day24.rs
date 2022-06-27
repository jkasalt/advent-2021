use std::collections::HashSet;
use std::{iter::Inspect, str::FromStr};

#[derive(Debug, Clone, Hash)]
enum Var {
    X,
    Y,
    Z,
    W,
}

impl FromStr for Var {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Var::X),
            "y" => Ok(Var::Y),
            "z" => Ok(Var::Z),
            "w" => Ok(Var::W),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Hash)]
enum Token {
    Var(Var),
    Int(i32),
}

impl Token {
    fn value_in(&self, state: &State) -> i32 {
        match self {
            Token::Var(Var::X) => state.x,
            Token::Var(Var::Y) => state.y,
            Token::Var(Var::Z) => state.z,
            Token::Var(Var::W) => state.w,
            Token::Int(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Hash)]
enum Ope {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone, Hash)]
struct Inst {
    ope: Ope,
    var: Var,
    token: Token,
}

impl Inst {
    fn new(ope: &str, var: &str, token: Option<&str>) -> Self {
        let ope = match ope {
            "inp" => Ope::Inp,
            "add" => Ope::Add,
            "mul" => Ope::Mul,
            "div" => Ope::Div,
            "mod" => Ope::Mod,
            "eql" => Ope::Eql,
            other => panic!("invalid input {other}"),
        };
        let var = Var::from_str(var).unwrap();
        let token = token.unwrap_or("x"); // Ugly!
        let token = if let Ok(n) = token.parse() {
            Token::Int(n)
        } else {
            Token::Var(Var::from_str(token).unwrap())
        };
        Inst { ope, var, token }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

type Inputs = Vec<i8>;

#[derive(Debug, Clone, Hash)]
pub struct Program {
    state: State,
    input_idx: usize,
    instr: Vec<Inst>,
}

impl Program {
    fn execute(&mut self, inputs: &Inputs, memory: &mut HashSet<(u64, State)>) -> bool {
        let mut will_memorize = Vec::new();
        for inst in &self.instr {
            // println!("{}", self.input_idx);
            let token_value = inst.token.value_in(&self.state);
            let ope_upon = match inst.var {
                Var::X => &mut self.state.x,
                Var::Y => &mut self.state.y,
                Var::Z => &mut self.state.z,
                Var::W => &mut self.state.w,
            };
            match inst.ope {
                Ope::Inp => {
                    *ope_upon = inputs[self.input_idx] as i32;
                    let num = as_usual_number(&inputs[self.input_idx..]);
                    if memory.contains(&(num, self.state.clone())) {
                        return false;
                    }
                    will_memorize.push((num, self.state.clone()));
                    self.input_idx += 1;
                    self.input_idx %= 14;
                }
                Ope::Add => *ope_upon += token_value,
                Ope::Mul => *ope_upon *= token_value,
                Ope::Div => *ope_upon /= token_value,
                Ope::Mod => *ope_upon %= token_value,
                Ope::Eql => *ope_upon = (*ope_upon == token_value) as i32,
            }
        }
        let is_valid = self.state.z == 0;
        if !is_valid {
            for substate in will_memorize {
                memory.insert(substate);
            }
        }
        println!("{:?}: {:?}", inputs, self.state);
        is_valid
    }
}

pub fn gen(input: &str) -> Program {
    let instr = input.lines().fold(Vec::new(), |mut v, line| {
        let mut words = line.split_whitespace();
        let ope = words.next().unwrap();
        let var = words.next().unwrap();
        let token = words.next();

        assert!(ope != "inp" || token.is_none());

        v.push(Inst::new(ope, var, token));
        v
    });
    let state = State {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    };
    Program {
        state,
        instr,
        input_idx: 0,
    }
}

struct NumIter {
    i: u64,
}

impl Iterator for NumIter {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 11111111111111 {
            None
        } else {
            self.i -= 1;
            while let Some(idx) = find_zero(self.i) {
                // dbg!(idx);
                self.i -= (self.i % 10_u64.pow(idx as u32)) + 1;
            }
            // println!("{}", self.i);
            Some(self.i)
        }
    }
}

pub fn first(program: &Program) -> u64 {
    let i = 99999999999999_u64;
    let mut memory = HashSet::new();
    let numbers = NumIter { i };
    for i in numbers {
        let mut prog = program.clone();
        if prog.execute(&as_model_number(i), &mut memory) {
            return i;
        }
    }
    panic!();
    // let best = (11111111111111..=99999999999999)
    //     .rev()
    //     .filter(|n| !n.to_string().contains('0'))
    //     .map(as_model_number)
    //     // .inspect(|i| println!("{i:?}"))
    //     .find(|mod_num| {
    //         let mut prog = program.clone();
    //         prog.execute(mod_num);
    //         prog.state.z == 0
    //     })
    //     .unwrap();
    // as_usual_number(best)
}

fn find_zero(num: u64) -> Option<usize> {
    (0..=13)
        .map(|exp| 10_u64.pow(exp))
        .map(|divisor| ((num / divisor) % 10))
        .position(|n| n == 0)
}

fn as_model_number(num: u64) -> Inputs {
    (0..=13)
        .map(|exp| 10_u64.pow(exp))
        .map(|div| ((num / div) % 10) as i8)
        .rev()
        .collect()
}

fn as_usual_number(num: &[i8]) -> u64 {
    num.iter()
        .rev()
        .enumerate()
        .fold(0, |int, (i, digit)| {
            int + *digit as u64 * 10_u64.pow(i as u32)
        })
}

pub fn second(program: &Program) -> u64 {
    0
}
