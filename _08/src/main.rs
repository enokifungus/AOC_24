use std::collections::{HashSet,HashMap};
use std::io::stdin;

const DATA:&str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

struct Grid {
    at: HashMap<u8,Vec<(i32,i32)>>,
    wd: i32,
    ht: i32,
}

impl Grid {
    fn new<I: Iterator<Item = String>>(sc:I) -> Self {
        let mut at = HashMap::new();
        let mut ht = 0;
        let mut wd = 0;
        for (r,s) in sc.enumerate() {
            ht += 1;
            wd = 0;
            for (c,b) in s.bytes().enumerate() {
                wd += 1;
                if b != b'.' {
                    at.entry(b).or_insert_with(Vec::new).push((r as i32,c as i32));
                }
            }
        }
        Self { at,wd,ht }
    }

    fn step(&self, q:(i32,i32), d:(i32,i32)) -> Option<(i32,i32)> {
        let r = (q.0 + d.0,q.1 + d.1);
        (0 <= r.0 && r.0 < self.wd && 0 <= r.1 && r.1 < self.ht).then_some(r)
    }

    fn pairs<I>(&self) -> I
        where
            I: Iterator<Item = (i32,i32)>
    {
        self.at.values()
            .flat_map(|v| (1..v.len()).flat_map(|j| (0..j).map(move |i| (v[i],v[j]))))
    }
}

fn main() {
    let sc = DATA.lines().map(str::to_string);
    //let sc = stdin().lines().map(Result::unwrap);
    let gd = Grid::new(sc);
    let mut nd = HashSet::new();
    for (p,q) in gd.pairs() {
        let d = (q.0 - p.0,q.1 - p.1);
        if let Some(r) = gd.step(q,d) { nd.insert(r); }
        let d = (-d.0,-d.1);
        if let Some(r) = gd.step(p,d) { nd.insert(r); }
    }
    let n1 = nd.len();
    println!("{n1}");
}
