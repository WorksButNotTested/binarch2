use crate::magic::{magics, Kind};
use std::collections::{hash_map::Iter, HashMap, HashSet};

type BinarchMatches = HashSet<usize>;
type BinarchResults = HashMap<Kind, BinarchMatches>;

#[derive(Default)]
pub struct Binarch {
    results: BinarchResults,
}

impl<'a> IntoIterator for &'a Binarch {
    type Item = (&'a Kind, &'a BinarchMatches);
    type IntoIter = Iter<'a, Kind, BinarchMatches>;
    fn into_iter(self) -> Iter<'a, Kind, BinarchMatches> {
        self.results.iter()
    }
}

impl Binarch {
    fn merge(a: &mut BinarchResults, kb: &Kind, vb: &BinarchMatches) {
        match a.get_mut(kb) {
            Some(va) => va.extend(vb),
            None => {
                a.insert(kb.clone(), vb.clone());
            }
        }
    }

    pub fn new(i: usize, ck: &[u8]) -> Binarch {
        let mut results = HashMap::<Kind, BinarchMatches>::new();
        for m in magics().iter() {
            let indexes = m
                .regex()
                .find_iter(ck)
                .filter(|c| m.matches(&ck[c.start()..c.end()]))
                .map(|c| i + c.start())
                .collect::<BinarchMatches>();
            Self::merge(&mut results, &m.kind(), &indexes);
        }
        Binarch { results }
    }

    pub fn reduce(a: Binarch, b: Binarch) -> Binarch {
        let mut result = a;
        for (kb, vb) in b.results {
            Self::merge(&mut result.results, &kb, &vb);
        }
        result
    }
}
