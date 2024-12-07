use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

#[derive(Debug)]
struct Map {
    m: Vec<Vec<u8>>,
    w: usize,
    h: usize,
}

impl Map {
    fn load(input: &str) -> Result<Map> {
        let mut data: Vec<Vec<u8>> = Vec::new();

        for l in input.lines() {
            data.push(l.trim().as_bytes().to_vec());
        }

        let w = data[0].len();
        let h = data.len();

        Ok(Map { m: data, w, h })
    }

    fn get(&self, x: i64, y: i64) -> Option<u8> {
        if (x < 0) || (y < 0) {
            return None;
        }

        if let Some(line) = self.m.get(y as usize) {
            if let Some(val) = line.get(x as usize) {
                return Some(*val);
            }
        }

        None
    }

    fn locate_guard(&self) -> Option<(i64, i64)> {
        for y in 0..self.h {
            for x in 0..self.w {
                if let Some(b'^') = self.get(x as i64, y as i64) {
                    return Some((x as i64, y as i64));
                }
            }
        }

        None
    }

    #[allow(unused)]
    fn print(v: &[Vec<u8>]) {
        for l in v.iter() {
            for x in l {
                print!("{}", *x as char)
            }
            println!();
        }
    }

    fn poscount(v: &[Vec<u8>]) -> usize {
        v.iter()
            .map(|l| l.iter().filter(|t| (**t == b'X') || (**t == b'^')).count())
            .sum()
    }

    fn free_at(&mut self, x: i64, y: i64) -> bool {
        if let Some(v) = self.get(x, y) {
            if v == b'#' {
                self.m[y as usize][x as usize] = b'.';
            }
        }
        false
    }

    fn obstacle_at(&mut self, x: i64, y: i64) -> bool {
        if let Some(v) = self.get(x, y) {
            if v == b'.' {
                self.m[y as usize][x as usize] = b'#';
                return true;
            }
        }
        false
    }

    fn execute(&self, lim: Option<usize>) -> usize {
        let mut m = self.m.clone();

        //Map::print(&m);

        let g = self.locate_guard();

        if g.is_none() {
            println!("Failed to find guard in map");
            return 0;
        }

        let mut g = g.unwrap();

        let mut w: (i64, i64) = (0, -1);

        let mut targ = (g.0 + w.0, g.1 + w.1);
        let mut prev = targ;

        let mut cnt: usize = 0;

        while let Some(v) = self.get(targ.0, targ.1) {
            match v {
                b'#' => {
                    /* We need to turn right */
                    //println!("From ({},{})", w.0, w.1);

                    w = match w {
                        (1, 0) => (0, 1),
                        (-1, 0) => (0, -1),
                        (0, 1) => (-1, 0),
                        (0, -1) => (1, 0),
                        _ => panic!("Unexpected vector"),
                    };
                    //println!("Turning ({},{})", w.0, w.1);
                    /* Go back */
                    g = prev;
                }
                b'.' | b'^' => {
                    /* Moving there */
                    //println!("Guard at ({},{}) targ ({}, {})", g.0, g.1, targ.0, targ.1);
                    g = targ;

                    m[g.1 as usize][g.0 as usize] = b'X';
                }
                _ => panic!("Unexpected value '{v}' in map"),
            }

            prev = targ;
            targ = (g.0 + w.0, g.1 + w.1);
            if let Some(lim) = lim {
                cnt += 1;
                if cnt > lim {
                    return 0;
                }
            }
        }
        //Map::print(&m);

        Map::poscount(&m)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.file)?;
    let mut reader = BufReader::new(f);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let mut m = Map::load(&data)?;

    let cnt = m.execute(Some(65536));

    println!("==> {}", cnt);

    let mut loop_cnt = 0;

    for x in 0..m.h {
        for y in 0..m.w {
            if m.obstacle_at(x as i64, y as i64) {
                let d = m.execute(Some(65536));

                //println!("==> {}", d);

                if d == 0 {
                    loop_cnt += 1;
                }

                m.free_at(x as i64, y as i64);
            }
        }
    }

    println!("==LOOP==> {}", loop_cnt);

    Ok(())
}
