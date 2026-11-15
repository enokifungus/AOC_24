use std::collections::{BinaryHeap,HashMap};
use std::io::stdin;

fn part1(sc: impl Iterator<Item = String>) -> (u32,i32) {
    let (mut lf,mut rt):(BinaryHeap<i32>,BinaryHeap<i32>) = sc.map(|i| {
        let (a,b) = i.split_once("   ").unwrap();
        (a.parse::<i32>().unwrap(),b.parse::<i32>().unwrap())
    }).unzip();
    let mut la:HashMap<i32,i32> = HashMap::new();
    for &x in &lf { *la.entry(x).or_default() += x; }
    let mut n1 = 0;
    let mut n2 = 0;
    while let Some(i) = lf.pop() {
        let j = rt.pop().unwrap();
        n1 += i.abs_diff(j);
        if let Some(a) = la.get(&j) { n2 += a; }
    }
    (n1,n2)
}

fn main() {
    let sc = stdin().lines().map(|i| i.unwrap());
    let (n1,n2) = part1(sc);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn _1() {
        let sc = DATA.lines().map(String::from);
        let (n1,n2) = part1(sc);
        assert_eq!(n1,11);
        assert_eq!(n2,31);
    }
}
