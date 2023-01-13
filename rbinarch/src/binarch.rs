use crate::magic::{magics, Kind};
use std::collections::{hash_map::Iter, HashMap, HashSet};

type BinarchMatches = HashSet<usize>;
type BinarchResults = HashMap<Kind, BinarchMatches>;

#[derive(Default)]
pub struct Binarch(BinarchResults);

impl<'a> IntoIterator for &'a Binarch {
    type Item = (&'a Kind, &'a BinarchMatches);
    type IntoIter = Iter<'a, Kind, BinarchMatches>;
    fn into_iter(self) -> Iter<'a, Kind, BinarchMatches> {
        self.0.iter()
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

    pub fn new(offset: usize, chunk: &[u8]) -> Binarch {
        let mut results = HashMap::<Kind, BinarchMatches>::new();
        for magic in magics().iter() {
            let matches = magic
                .regex()
                .find_iter(chunk)
                .filter(|rmatch| magic.matches(&chunk[rmatch.start()..rmatch.end()]))
                .map(|rmatch| offset + rmatch.start())
                .collect::<BinarchMatches>();
            Self::merge(&mut results, &magic.kind(), &matches);
        }
        Binarch(results)
    }

    pub fn reduce(a: Binarch, b: Binarch) -> Binarch {
        let mut result = a;
        for (kb, vb) in b.0 {
            Self::merge(&mut result.0, &kb, &vb);
        }
        result
    }
}
