use std::io::{stdin,read_to_string,BufRead,BufReader};
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

    fn get_num(bu:&[u8], tm:u8) -> Result<(i32,usize),usize> {
        let o = bu.iter().position(|c| !(b'0'<=*c && *c<=b'9'));
        if let Some(i) = o {
            if bu[i] == tm {
                Ok((str::from_utf8(&bu[..i]).unwrap().parse::<i32>().unwrap(),i + 1))
            } else {
                Err(i)
            }
        } else {
            Err(bu.len())
        }
    }

    #[derive(Debug)]
    enum Mul {
        Do,
        Dont,
        Product(i32),
    }

    struct ParseMul<T: BufRead> {
        br: T,
    }

    impl<T: BufRead> Iterator for ParseMul<T> {
        type Item = Mul;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let bu = self.br.fill_buf().unwrap();
                if bu.is_empty() {
                    return None;
                }
                let mut r = None;
                let k = if bu.starts_with(b"do()") {
                    r = Some(Mul::Do);
                    4
                } else if bu.starts_with(b"don't()") {
                    r = Some(Mul::Dont);
                    7
                } else if bu.starts_with(b"mul(") {
                    let bu = &bu[4..];
                    4 + match get_num(bu,b',') {
                        Ok((v1,i)) => {
                            let bu = &bu[i..];
                            i + match get_num(bu,b')') {
                                Ok((v2,j)) => {
                                    r = Some(Mul::Product(v1*v2));
                                    j
                                }
                                Err(j) => {
                                    j
                                }
                            }
                        }
                        Err(i) => {
                            i
                        }
                    }
                } else {
                    1
                };
                self.br.consume(k);
                if r.is_some() { return r; }
            }
        }
    }

    #[test]
    fn _2() {
        let mut pm = ParseMul { br:BufReader::new(DATA.as_bytes()) };
        for i in pm { println!("{i:?}"); }
    }
}
