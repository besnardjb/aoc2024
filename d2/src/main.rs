use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

struct Report {
    vals: Vec<u64>,
}

impl Report {
    fn new(data: &str) -> Report {
        let vals: Vec<u64> = data.split(" ").map(|v| v.parse().unwrap()).collect();
        Report { vals }
    }

    fn safe(&self) -> bool {
        let mut increasing: Option<Ordering> = None;

        for (v1, v2) in self.vals.iter().zip(self.vals.iter().skip(1)) {
            /* Check ORDER */
            let cmp = v1.cmp(v2);

            if let Some(pcmp) = increasing {
                if cmp != pcmp {
                    return false;
                }
            } else {
                increasing = Some(cmp);
            }

            /* Check increase */
            if (v1.abs_diff(*v2) > 3) || (v1 == v2) {
                return false;
            }
        }
        true
    }

    fn subset(&self, ti: usize) -> Report {
        let vals = self
            .vals
            .iter()
            .enumerate()
            .filter_map(|(i, val)| if i == ti { None } else { Some(*val) })
            .collect();
        Report { vals }
    }

    fn safe_minus_one(&self) -> bool {
        for i in 0..self.vals.len() {
            let candi = self.subset(i);
            if candi.safe() {
                return true;
            }
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

    let reports = data.lines().map(Report::new).collect::<Vec<Report>>();

    let safe = reports.iter().filter(|v| v.safe()).count();

    println!("{} reports are safe", safe);

    let safe_dist_1 = reports.iter().filter(|v| v.safe_minus_one()).count();

    println!("{} reports are safe D1", safe_dist_1);

    Ok(())
}
