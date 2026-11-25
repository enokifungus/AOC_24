use std::io::stdin;

fn find_formula_2(tg:i64, v:&[i64]) -> bool {
    let x = v[0];
    if v.len() == 1 {
        tg == x
    } else if tg%x == 0 && find_formula_2(tg/x,&v[1..]) || tg >= x && find_formula_2(tg - x,&v[1..]) {
        true
    } else {
        let a = 10i64.pow(x.ilog10() + 1);
        tg%a == x && find_formula_2(tg/a,&v[1..])
    }
}

fn find_formula(tg:i64, v:&[i64]) -> bool {
    let x = v[0];
    if v.len() == 1 {
        tg == x
    } else {
        tg%x == 0 && find_formula(tg/x,&v[1..]) || tg >= x && find_formula(tg - x,&v[1..])
    }
}

fn solve<I: Iterator<Item=String>>(sc:I) -> (i64,i64) {
    let mut n1 = 0;
    let mut n2 = 0;
    for i in sc {
        let (tg,v) = i.split_once(": ").unwrap();
        let tg = tg.parse().unwrap();
        let mut v:Vec<_> = v.split(' ').map(|s| s.parse().unwrap()).collect();
        v.reverse();
        if find_formula(tg,&v) { n1 += tg; }
        if find_formula_2(tg,&v) { n2 += tg; }
    }
    (n1,n2)
}

fn main() {
    let sc = stdin().lines().map(|r| r.unwrap());
    let (n1,n2) = solve(sc);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA:&str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn _1() {
        let sc = DATA.lines().map(str::to_string);
        let (n1,n2) = solve(sc);
        assert_eq!(n1,3749);
        assert_eq!(n2,11387);
    }
}
