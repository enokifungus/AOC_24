use std::io::stdin;

fn is_report_safe(rp:&[i32]) -> bool {
    let mut t2 = rp.windows(2).map(|a| {
        let [x,y] = a.try_into().unwrap();
        let d = x.abs_diff(y);
        if 1<=d && d<=3 { Some(x>y) } else { None }
    });
    if let Some(Some(b1)) = t2.next() {
        if t2.all(|p| p.is_some_and(|b2| b1 == b2)) { return true; }
    }
    false
}

fn part1(sc:impl Iterator<Item=String>) -> (i32,i32) {
    let mut n1 = 0;
    let mut n2 = 0;
    for s in sc {
        let mut rp:Vec<i32> = s.split(' ').map(|t| t.parse().unwrap()).collect();
        if is_report_safe(&rp) {
            n1 += 1;
            n2 += 1;
        } else if (0..rp.len()).any(|i| {
            let x = rp.remove(i);
            if is_report_safe(&rp) { return true; }
            rp.insert(i,x);
            false
        }) {
            n2 += 1;
        }
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

    const DATA:&str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn _1() {
        let sc = DATA.lines().map(String::from);
        let (n1,n2) = part1(sc);
        assert_eq!(n1,2);
        assert_eq!(n2,4);
    }
}
