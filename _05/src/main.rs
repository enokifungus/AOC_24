use std::collections::HashSet;
use std::io::stdin;

const DATA:&str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() {
    let mut sc = DATA.lines();
    let mut sc = stdin().lines().map(|i| i.unwrap());
    let pa:HashSet<(u8,u8)> = (0..).map(|_| sc.next().unwrap())
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let v:Vec<u8> = s.split('|').map(|i| i.parse().unwrap()).collect();
            (v[0],v[1])
        })
        .collect();
    let mut n1 = 0;
    for s in sc {
        let v:Vec<u8> = s.split(',').map(|i| i.parse().unwrap()).collect();
        let mut it = (1..v.len()).flat_map(|j| (0..j).map(move |i| (i,j)));
        if it.all(|(i,j)| !pa.contains(&(v[j],v[i]))) {
            n1 += v[v.len()/2] as i32;
        }
    }
    println!("{n1}");
}
