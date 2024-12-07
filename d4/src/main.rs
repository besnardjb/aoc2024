use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

struct Xmap {
    data: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl Xmap {
    fn new(data: &str) -> Xmap {
        let data: Vec<Vec<char>> = data.lines().map(|v| v.chars().collect()).collect();
        let w = data[0].len();
        let h = data.len();
        Xmap { data, w, h }
    }

    fn get(&self, x: i64, y: i64) -> Option<char> {
        if (x < 0) || (y < 0) {
            return None;
        }

        if let Some(l) = self.data.get(y as usize) {
            if let Some(v) = l.get(x as usize) {
                return Some(*v);
            }
        }
        None
    }

    fn views_at(&self, x: usize, y: usize) -> Vec<String> {
        let mut rets: Vec<String> = Vec::new();

        for dx in [-1i64, 0, 1] {
            for dy in [-1i64, 0, 1] {
                if (dy == 0) && (dx == 0) {
                    continue;
                }

                let mut candi = String::new();

                for i in 0.."XMAS".len() {
                    let tx = x as i64 + i as i64 * dx;
                    let ty = y as i64 + i as i64 * dy;

                    if (tx < 0) || (ty < 0) {
                        continue;
                    }

                    if let Some(c) = self.get(tx, ty) {
                        candi.push_str(&c.to_string());
                    } else {
                        break;
                    }
                }

                rets.push(candi);
            }
        }

        rets
    }

    fn xmas_at(&self, x: usize, y: usize) -> usize {
        self.views_at(x, y).iter().filter(|v| **v == "XMAS").count()
    }

    fn xfind(&self, x: usize, y: usize) -> bool {
        /* a  b
         e
        c d */
        let x = x as i64;
        let y = y as i64;

        let a = self.get(x - 1, y - 1).unwrap_or(' ');
        let b = self.get(x + 1, y - 1).unwrap_or(' ');
        let c = self.get(x - 1, y + 1).unwrap_or(' ');
        let d = self.get(x + 1, y + 1).unwrap_or(' ');

        let e = self.get(x, y).unwrap_or(' ');

        //println!("{} {}\n {} \n{} {}\n", a, b, e, c, d);

        if e != 'A' {
            return false;
        }

        let sa: String = [a, e, d].iter().collect();
        let sb: String = [b, e, c].iter().collect();

        if (["MAS", "SAM"].contains(&sa.as_str())) && (["MAS", "SAM"].contains(&sb.as_str())) {
            return true;
        }

        false
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.file)?;
    let mut reader = BufReader::new(f);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let map = Xmap::new(&data);

    let mut xmas_cnt = 0;

    for x in 0..map.w {
        for y in 0..map.h {
            xmas_cnt += map.xmas_at(x, y);
        }
    }

    println!(" ==> {}", xmas_cnt);

    let mut xmas_cnt = 0;

    for x in 0..map.w {
        for y in 0..map.h {
            if map.xfind(x, y) {
                xmas_cnt += 1;
            }
        }
    }

    println!(" ==> {}", xmas_cnt);

    Ok(())
}
