use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"Player \d+ starting position: (\d+)").unwrap());

type Wincounts = HashMap<(Players, bool), (u128, u128)>;

static HASH_MAP: Lazy<Mutex<Wincounts>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Player {
    points: u32,
    position: u32,
}

impl Player {
    fn handle_roll(&mut self, roll: u32) {
        self.position = (self.position + roll) % 10;
        self.points += match self.position {
            0 => 10,
            n => n,
        };
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Players {
    player0: Player,
    player1: Player,
}

impl std::ops::Index<bool> for Players {
    type Output = Player;
    fn index(&self, index: bool) -> &Self::Output {
        if index {
            &self.player1
        } else {
            &self.player0
        }
    }
}

impl std::ops::IndexMut<bool> for Players {
    fn index_mut(&mut self, index: bool) -> &mut Self::Output {
        if index {
            &mut self.player1
        } else {
            &mut self.player0
        }
    }
}

fn play_deter(players: &mut [Player], max_points: u32) -> (usize, usize) {
    for t in 0.. {
        let n = (3 * t as u32) % 100;
        let player = &mut players[t % 2];
        player.handle_roll(3 * n + 6);
        if player.points >= max_points {
            return ((t % 2) + 1, (t + 1) * 3);
        }
    }
    unreachable!()
}

const DICE: [[u32; 3]; 27] = [
    [3, 1, 1],
    [3, 1, 2],
    [3, 1, 3],
    [3, 2, 1],
    [3, 2, 2],
    [3, 2, 3],
    [3, 3, 1],
    [3, 3, 2],
    [3, 3, 3],
    [2, 1, 1],
    [2, 1, 2],
    [2, 1, 3],
    [2, 2, 1],
    [2, 2, 2],
    [2, 2, 3],
    [2, 3, 1],
    [2, 3, 2],
    [2, 3, 3],
    [1, 1, 1],
    [1, 1, 2],
    [1, 1, 3],
    [1, 2, 1],
    [1, 2, 2],
    [1, 2, 3],
    [1, 3, 1],
    [1, 3, 2],
    [1, 3, 3],
];

fn play_dirac(players: Players, turn: bool) -> (u128, u128) {
    let mut win_count = (0, 0);
    for dice in DICE {
        let mut sub_players = players; // Copy players state
        let roll = dice.iter().sum();
        sub_players[turn].handle_roll(roll);
        // Check if somebody won
        if sub_players[turn].points >= 21 {
            if turn {
                win_count.1 += 1;
            } else {
                win_count.0 += 1;
            }
        } else {
            // Dont recurse if we already know the outcome
            let map = HASH_MAP.lock().unwrap();
            let sub_win_count = if let Some(res) = map.get(&(sub_players, turn)) {
                *res
            } else {
                // Otherwise recurse with the other player rolling dice
                drop(map);
                let res = play_dirac(sub_players, !turn);
                let mut map = HASH_MAP.lock().unwrap();
                map.insert((sub_players, turn), res);
                res
            };
            win_count.0 += sub_win_count.0;
            win_count.1 += sub_win_count.1;
        }
    }
    win_count
}

pub fn first(input: &str) -> usize {
    let mut players = Vec::new();
    for cap in RE.captures_iter(input) {
        let position = cap[1].parse().unwrap();
        players.push(Player {
            points: 0,
            position,
        });
    }
    let (loser, t_max) = play_deter(&mut players, 1000);
    players[loser].points as usize * t_max
}

pub fn second(input: &str) -> usize {
    let mut players = Vec::new();
    for cap in RE.captures_iter(input) {
        let position = cap[1].parse().unwrap();
        players.push(Player {
            points: 0,
            position,
        });
    }
    let players = Players {
        player0: players[0],
        player1: players[1],
    };
    let res = play_dirac(players, false);
    if res.0 > res.1 {
        res.0.try_into().unwrap()
    } else {
        res.1.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
        assert_eq!(first(input), 739785);
    }
    #[test]
    fn part2() {
        let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
        assert_eq!(second(input), 444356092776315);
    }
}
