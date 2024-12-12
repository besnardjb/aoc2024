use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

struct AntMap {
    m: Vec<Vec<u8>>,
    w: i64,
    h: i64,
}

impl AntMap {
    fn new(data: &str) -> AntMap {
        let mut m = Vec::new();
        for l in data.lines() {
            m.push(l.as_bytes().to_vec())
        }

        AntMap {
            w: m[0].len() as i64,
            h: m.len() as i64,
            m,
        }
    }

    fn set(m: &mut [Vec<u8>], x: i64, y: i64, val: u8) -> bool {
        if (x < 0) || (y < 0) {
            return false;
        }

        if let Some(l) = m.get_mut(y as usize) {
            if let Some(v) = l.get_mut(x as usize) {
                *v = val;
                return true;
            }
        }

        false
    }

    fn get(&self, x: i64, y: i64) -> Option<char> {
        if (x < 0) || (y < 0) {
            return None;
        }

        if let Some(l) = self.m.get(y as usize) {
            if let Some(v) = l.get(x as usize) {
                return Some(*v as char);
            }
        }

        None
    }

    fn antenna_network(&self) -> HashMap<char, Vec<(i64, i64)>> {
        let mut ret: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(v) = self.get(x, y) {
                    if v != '.' {
                        ret.entry(v).or_default().push((x, y));
                    }
                } else {
                    unreachable!("We should be in array");
                }
            }
        }

        ret
    }

    fn in_bound(&self, (x, y): &(i64, i64)) -> bool {
        ((0 <= *y) && (*y < self.h)) && ((0 <= *x) && (*x < self.w))
    }

    fn antinodes(&self, coords: &[(i64, i64)], inline: bool) -> HashSet<(i64, i64)> {
        let mut ret = HashSet::new();

        for (x1, y1) in coords.iter() {
            for (x2, y2) in coords.iter() {
                let dx = x2 - x1;
                let dy = y2 - y1;

                if (x1 == x2) && (y1 == y2) {
                    continue;
                }

                if !inline {
                    let t1 = (x1 - dx, y1 - dy);
                    if self.in_bound(&(t1.0, t1.1)) {
                        ret.insert(t1);
                    }
                    let t2 = (x2 + dx, y2 + dy);

                    if self.in_bound(&(t2.0, t2.1)) {
                        ret.insert(t2);
                    }
                } else {
                    let mut t1 = (x1 - dx, y1 - dy);

                    while self.in_bound(&(t1.0, t1.1)) {
                        ret.insert(t1);
                        t1 = (t1.0 - dx, t1.1 - dy);
                    }

                    let mut t2 = (x1 + dx, y1 + dy);

                    while self.in_bound(&(t2.0, t2.1)) {
                        ret.insert(t2);
                        t2 = (t2.0 + dx, t2.1 + dy);
                    }
                }
            }
        }

        ret
    }

    fn print(&self, anti: Option<&HashSet<(i64, i64)>>) {
        let m = if let Some(anti) = anti {
            let mut c = self.m.clone();

            for (x, y) in anti {
                AntMap::set(&mut c, *x, *y, b'#');
            }

            c
        } else {
            self.m.clone()
        };

        for l in m.iter() {
            for v in l.iter() {
                print!("{}", *v as char);
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.file)?;
    let mut reader = BufReader::new(f);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let m = AntMap::new(&data);

    let ants = m.antenna_network();

    let mut anti: HashSet<(i64, i64)> = HashSet::new();

    for a in ants.values() {
        let lanti = m.antinodes(a, false);
        anti.extend(lanti);
    }

    m.print(Some(&anti));

    println!("LEN {}", anti.len());

    let mut anti: HashSet<(i64, i64)> = HashSet::new();

    for a in ants.values() {
        let lanti = m.antinodes(a, true);
        anti.extend(lanti);
    }

    m.print(Some(&anti));

    println!("LEN {}", anti.len());

    Ok(())
}
