use std::io::{stdin,read_to_string};
use regex::Regex;

fn part1(sc: &str) -> (i32,i32) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(d)o\(\)|(d)on't\(\)").unwrap();
    let mut n1 = 0;
    let mut n2 = 0;
    let mut b = true;
    for m in re.captures_iter(sc) {
        if m.get(3).is_some() {
            b = true;
        } else if m.get(4).is_some() {
            b = false;
        } else {
            let x = m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap();
            n1 += x;
            if b { n2 += x; }
        }
        
    }
    (n1,n2)
}

fn main() {
    let sc = &read_to_string(stdin()).unwrap();
    let (n1,n2) = part1(sc);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn _1() {
        let sc = DATA;
        let (n1,n2) = part1(sc);
        assert_eq!(n1,161);
        assert_eq!(n2,48);
    }
}
