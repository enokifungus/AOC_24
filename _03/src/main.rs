use std::io::{ stdin, Read };

const DATA:&[u8] = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn part1(mut sc: impl Iterator<Item = u8>) -> i32 {
    let mut n1 = 0;
    let cd = b"mul(01,01)";
    let mut cx = 0;
    let mut pb = None;
    let mut cg = Vec::new();
    let mut cv = String::new();
    while let Some(c) = pb.take().or_else(|| sc.next()) {
        match cd[cx] {
            b'0' => {
                if b'0'<=c && c<=b'9' {
                    cx += 1;
                    cv.clear();
                    cv.push(c as char);
                } else {
                    cx = 0;
                    pb = Some(c);
                }
            },
            b'1' => {
                if b'0'<=c && c<=b'9' {
                    cv.push(c as char);
                } else {
                    cg.push(cv.parse::<i32>().unwrap());
                    cx += 1;
                    pb = Some(c);
                }
            },
            _ => {
                if c == cd[cx] {
                    if cx == 0 { cg.clear(); }
                    cx += 1;
                    if cx == cd.len() {
                        cx = 0;
                        n1 += cg[0]*cg[1];
                    }
                } else if cx != 0 {
                    cx = 0;
                    pb = Some(c);
                }
            }
        }
    }
    n1
}

fn main() {
    let sc = stdin().bytes().map(|c| c.unwrap());
    let n1 = part1(sc);
    println!("{n1}");
}
