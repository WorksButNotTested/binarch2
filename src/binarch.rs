use crate::magic::{magics, Kind};
use std::collections::{HashMap, HashSet};

pub type BinarchMatches = HashSet<usize>;
pub type BinarchResults = HashMap<Kind, BinarchMatches>;

pub struct Binarch;

impl Binarch {
    fn merge(a: &mut BinarchResults, kb: &Kind, vb: &BinarchMatches) {
        match a.get_mut(kb) {
            Some(va) => va.extend(vb),
            None => {
                a.insert(kb.clone(), vb.clone());
            }
        }
    }

    pub fn reduce(a: BinarchResults, b: BinarchResults) -> BinarchResults {
        let mut result = a;
        for (kb, vb) in b {
            Self::merge(&mut result, &kb, &vb);
        }
        result
    }

    pub fn process(i: usize, ck: &[u8]) -> BinarchResults {
        let mut h = BinarchResults::new();
        for m in magics().iter() {
            let indexes = m
                .regex()
                .find_iter(ck)
                .filter(|c| m.matches(&ck[c.start()..c.end()]))
                .map(|c| i + c.start())
                .collect::<HashSet<usize>>();
            Self::merge(&mut h, &m.kind(), &indexes);
        }
        h
    }

    pub fn default() -> BinarchResults {
        BinarchResults::new()
    }
}
