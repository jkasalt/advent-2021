use regex::Regex;
use once_cell::sync::Lazy;
use std::sync::Mutex;

type MaybeList = Option<Vec<(i32, i32, i32)>>;
static HAS_WORKED: Lazy<Mutex<MaybeList>> = Lazy::new(|| Mutex::new(None));

fn compute(input: &str) -> Vec<(i32, i32, i32)> {
    let re = Regex::new(r"x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    let x_min: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
    let x_max: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
    let y_min: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
    let y_max: i32 = cap.get(4).unwrap().as_str().parse().unwrap();
    let mut has_worked = Vec::new();

    for v_x_0 in -1000..1000 {
        'throw: for v_y_0 in -1000..1000 {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            let mut v_x = v_x_0;
            let mut v_y = v_y_0;
            let mut max_height = v_y;
            while x.abs() < 10000 && y.abs() < 10000 {
                x += v_x;
                y += v_y;
                if y > max_height {
                    max_height = y;
                }
                if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
                    has_worked.push((max_height, v_x_0, v_y_0));
                    continue 'throw;
                }
                v_x -= v_x.signum();
                v_y -= 1;
            }
        }
    }
    has_worked
}

pub fn first(input: &str) -> i32 {
    let maybe_list = &mut *HAS_WORKED.lock().unwrap();
    if maybe_list.is_none() {
        *maybe_list = Some(compute(input));
    } 
    maybe_list.as_ref().unwrap().iter().map(|z| z.0).max().unwrap()
}

pub fn second(input: &str) -> i32 {
    let maybe_list = &mut *HAS_WORKED.lock().unwrap();
    if maybe_list.is_none() {
        *maybe_list = Some(compute(input));
    } 
    maybe_list.as_ref().unwrap().len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(first(input), 45);
    }

    #[test]
    fn two() {
        let input = "target area: x=20..30, y=-10..-5";
        assert_eq!(second(input), 112);
    }
}
