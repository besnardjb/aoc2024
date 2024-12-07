use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

#[derive(Clone, Debug)]
struct OrderingRule {
    id: u64,
    predecessors: HashSet<u64>,
    successors: HashSet<u64>,
}

impl OrderingRule {
    fn new(id: u64) -> OrderingRule {
        OrderingRule {
            predecessors: HashSet::new(),
            successors: HashSet::new(),
            id,
        }
    }

    fn prune(&mut self, ord: &[u64]) {
        self.predecessors.retain(|v| ord.contains(v));
        self.successors.retain(|v| ord.contains(v));
    }
}

struct OrderingRules {
    rules: HashMap<u64, OrderingRule>,
}

impl OrderingRules {
    fn init() -> OrderingRules {
        OrderingRules {
            rules: HashMap::new(),
        }
    }

    fn push(&mut self, rule: &str) -> Result<()> {
        let ab: Vec<u64> = rule.split('|').filter_map(|v| v.parse().ok()).collect();

        if ab.len() != 2 {
            return Err(anyhow!("{} ==> {:?} Should be 2 elems", rule, ab));
        }

        {
            let a = self.rules.entry(ab[0]).or_insert(OrderingRule::new(ab[0]));
            a.successors.insert(ab[1]);
        }

        {
            let b = self.rules.entry(ab[1]).or_insert(OrderingRule::new(ab[1]));
            b.predecessors.insert(ab[0]);
        }

        Ok(())
    }

    fn check_order(&self, vals: &[u64]) -> bool {
        for (k, v) in vals.iter().enumerate() {
            if let Some(cur) = self.rules.get(v) {
                /* Check preds */
                for pred in &vals[0..k] {
                    if !cur.predecessors.contains(pred) {
                        return false;
                    }
                }

                for succs in &vals[k + 1..] {
                    if !cur.successors.contains(succs) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn prunning(&self, ord: &[u64]) -> OrderingRules {
        let mut new_rules = self.rules.clone();
        new_rules.retain(|k, _| ord.contains(k));

        for o in new_rules.values_mut() {
            o.prune(ord);
        }

        OrderingRules { rules: new_rules }
    }

    fn get_first(&self) -> Option<OrderingRule> {
        for v in self.rules.values() {
            if v.predecessors.is_empty() {
                return Some(v.clone());
            }
        }

        None
    }

    fn reorder(&self, vals: &[u64]) -> Vec<u64> {
        let pruned = self.prunning(vals);

        if let Some(f) = pruned.get_first() {
            if let Some(r) = pruned.walk(&[], f.id, 0, vals.len()) {
                return r;
            } else {
                panic!("Not expexted");
            }
        } else {
            println!("Error man");
        }

        vec![]
    }

    fn walk(&self, cur_ord: &[u64], id: u64, len: usize, tlen: usize) -> Option<Vec<u64>> {
        let mut ret: Vec<u64> = cur_ord.to_vec();
        ret.push(id);

        //println!("{:?} ({len}/{tlen})", ret);

        if (tlen - 1) == len {
            println!("CHECK ! {:?} {:?}", ret, self.check_order(cur_ord));

            if self.check_order(cur_ord) {
                return Some(ret);
            } else {
                return None;
            }
        }

        let cur = self.rules.get(&id).unwrap();

        //println!("CUR {:?}", cur);

        if len == 1 {
            let r: Vec<Vec<u64>> = cur
                .successors
                .par_iter()
                .filter_map(|e| self.walk(&ret, *e, len + 1, tlen))
                .collect();

            if !r.is_empty() {
                return Some(r[0].clone());
            }
        } else {
            for e in &cur.successors {
                if let Some(r) = self.walk(&ret, *e, len + 1, tlen) {
                    return Some(r);
                }
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let f = File::open(args.file)?;
    let mut reader = BufReader::new(f);
    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let mut rules = OrderingRules::init();

    let lines: Vec<&str> = data.lines().collect();

    let mut str_rules: Vec<&str> = Vec::new();
    let mut str_orders: Vec<&str> = Vec::new();

    let mut is_rule = true;

    for l in lines {
        if l.is_empty() {
            is_rule = false;
            continue;
        }

        if is_rule {
            str_rules.push(l.trim());
        } else {
            str_orders.push(l.trim());
        }
    }

    for l in str_rules {
        rules.push(l)?;
    }

    let mut valid_orders: Vec<Vec<u64>> = Vec::new();
    let mut invalid_orders: Vec<Vec<u64>> = Vec::new();

    for o in str_orders {
        let ord: Vec<u64> = o.split(",").map(|v| v.parse().unwrap()).collect();

        if rules.check_order(&ord) {
            valid_orders.push(ord);
        } else {
            invalid_orders.push(ord);
        }
    }

    let sum: u64 = valid_orders.iter().map(|v| v[v.len() / 2]).sum();

    println!("SUM is {}", sum);

    let mut reordered_vecs: Vec<Vec<u64>> = Vec::new();

    for i in invalid_orders {
        println!("WORKING on {:?}", i);
        reordered_vecs.push(rules.reorder(&i));
    }

    let sum: u64 = reordered_vecs.iter().map(|v| v[v.len() / 2]).sum();

    println!("PART2 SUM is {}", sum);

    Ok(())
}
