use std::io::{stdin,read_to_string};

const L4:[[(isize,isize);4];8] = [
    [(0,0),(1,0),(2,0),(3,0)],
    [(0,0),(1,1),(2,2),(3,3)],
    [(0,0),(0,1),(0,2),(0,3)],
    [(0,0),(-1,1),(-2,2),(-3,3)],
    [(0,0),(-1,0),(-2,0),(-3,0)],
    [(0,0),(-1,-1),(-2,-2),(-3,-3)],
    [(0,0),(0,-1),(0,-2),(0,-3)],
    [(0,0),(1,-1),(2,-2),(3,-3)],
];

const LX:[[(isize,isize);4];4] = [
    [(-1,-1),(1,1),(-1,1),(1,-1)],
    [(-1,-1),(1,1),(1,-1),(-1,1)],
    [(1,1),(-1,-1),(-1,1),(1,-1)],
    [(1,1),(-1,-1),(1,-1),(-1,1)],
];

const DATA:&str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

fn get_str(sc:&[&[u8]], r:usize, c:usize, l:&[(isize,isize)]) -> String {
    l.iter()
        .map_while(|(dr,dc)| r.checked_add_signed(*dr)
            .and_then(|r| c.checked_add_signed(*dc).map(|c| (r,c)))
            .and_then(|(r,c)| sc.get(r).and_then(|a| a.get(c).map(|b| *b as char))))
        .collect::<String>()
}

fn part1(sc:&str) -> (i32,i32) {
    let mut n1 = 0;
    let mut n2 = 0;
    let sc:Vec<_> = sc.lines().map(|s| s.as_bytes()).collect();
    for c in 0..sc[0].len() {
        for r in 0..sc.len() {
            for l in L4.iter() {
                if get_str(&sc,r,c,l) == "XMAS" {
                    n1 += 1;
                }
            }
            if sc[r][c] == b'A' && LX.iter().any(|l| get_str(&sc,r,c,l) == "MSMS") {
                n2 += 1;
            }
        }
    }
    (n1,n2)
}

fn main() {
    let sc = DATA;
    let sc = &read_to_string(stdin()).unwrap();
    let (n1,n2) = part1(sc);
    println!("PART1: {n1}");
    println!("PART2: {n2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _1() {
        let sc = DATA;
        let (n1,n2) = part1(sc);
        assert_eq!(n1,18);
        assert_eq!(n2,9);
    }
}
