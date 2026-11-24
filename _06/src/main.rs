use std::collections::HashSet;
use std::io::{stdin,read_to_string};

fn find_path(sc:&Vec<Vec<u8>>) -> Option<HashSet<(u8,u8)>> {
    let (r0,c0) = sc.iter().enumerate()
        .filter_map(|(r,s)| s.iter().position(|b| *b==b'^').map(|c| (r,c)))
        .next().unwrap();
    let (mut r,mut c) = (r0,c0);
    let da = [(-1,0),(0,1),(1,0),(0,-1)];
    let mut di = 0;
    let mut dp = 0;
    let mut vz = HashSet::new();
    let rz = 'a: loop {
        let pz = (r as u8,c as u8,di as u8);
        if !vz.insert(pz) { break 'a false; }
        loop {
            let op = r.checked_add_signed(da[di].0)
                .zip(c.checked_add_signed(da[di].1));
            let os = op.and_then(|(r,c)| sc.get(r).and_then(|a| a.get(c)));
            if let Some(s) = os {
                if *s != b'#' {
                    dp = di;
                    (r,c) = op.unwrap();
                    break;
                }
                di = (di + 1)%4;
                if di == dp { break 'a false; }
            } else {
                break 'a true;
            }
        }
    };
    rz.then(|| {
        let mut hs = vz.into_iter().map(|(r,c,_)| (r,c)).collect::<HashSet<_>>();
        hs.remove(&(r0 as u8,c0 as u8));
        hs
    })
}

fn solve(sc:impl Iterator<Item = String>) -> (usize,usize) {
    let mut sc:Vec<Vec<u8>> = sc.map(|s| s.into_bytes()).collect();
    let pa = find_path(&sc).unwrap();
    let n1 = pa.len() + 1;
    let mut n2 = 0;
    for (r,c) in pa {
        sc[r as usize][c as usize] = b'#';
        if find_path(&sc).is_none() { n2 += 1; }
        sc[r as usize][c as usize] = b'.';
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

    const DATA:&str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn _1() {
        let sc = DATA.lines().map(str::to_string);
        let (n1,n2) = solve(sc);
        assert_eq!(n1,41);
        assert_eq!(n2,6);
    }
}
