use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

fn compute_over(data: &str) -> Result<u64> {
    let mut tot = 0;
    let re: regex::Regex = regex::Regex::new("mul\\(([0-9]+),([0-9]+)\\)")?;

    for m in re.captures_iter(data) {
        println!("{:?}", m);

        let first = m.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let second = m.get(2).unwrap().as_str().parse::<u64>().unwrap();

        tot += first * second;
    }

    Ok(tot)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.file)?;
    let mut reader = BufReader::new(f);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let tot = compute_over(&data)?;
    println!("TOT is {}", tot);

    let mut dont_tot = 0;

    /* Use a clear entry to outline the 'do' */
    let data = data.clone().replace("do()", "£");
    let re = regex::Regex::new("don\\'t\\(\\)[^£]*")?;

    for m in re.find_iter(&data) {
        dont_tot += compute_over(m.as_str())?;
    }

    println!("DONT TOT is {}", dont_tot);
    println!("RES {}", tot - dont_tot);

    Ok(())
}
