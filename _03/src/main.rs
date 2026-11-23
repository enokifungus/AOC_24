use std::io::{stdin,read_to_string,BufRead,BufReader};

fn match_bytes(bu:&[u8], pa:&[u8]) -> Option<usize> {
    bu.starts_with(pa).then(|| pa.len())
}

fn match_num(bu:&[u8], tm:u8) -> (usize,Option<i32>) {
    if let Some(i) = bu.iter().position(|x| !(b'0'<=*x && *x<=b'9')) {
        if bu[i] == tm {
            if let Ok(v) = str::from_utf8(&bu[..i]).unwrap().parse() {
                (i + 1,Some(v))
            } else {
                (i + 1,None)
            }
        } else {
            (i, None)
        }
    } else {
        (bu.len(),None)
    }
}

fn part1(sc: &str) -> (i32,i32) {
    let mut sc = BufReader::new(sc.as_bytes());
    let mut dd = true;
    let mut n1 = 0;
    let mut n2 = 0;
    loop {
        let bu = sc.fill_buf().unwrap();
        if bu.is_empty() { break; }
        let k = if let Some(k) = match_bytes(bu,b"do()") {
            dd = true;
            k
        } else if let Some(k) = match_bytes(bu,b"don't()") {
            dd = false;
            k
        } else if let Some(k1) = match_bytes(bu,b"mul(") {
            let bu = &bu[k1..];
            let (k2,o) = match_num(bu,b',');
            if let Some(v1) = o {
                let bu = &bu[k2..];
                let (k3,o) = match_num(bu,b')');
                if let Some(v2) = o {
                    let v = v1*v2;
                    n1 += v;
                    if dd { n2 += v; }
                }
                k1 + k2 + k3
            } else {
                k1 + k2
            }
        } else {
            1
        };
        sc.consume(k);
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
