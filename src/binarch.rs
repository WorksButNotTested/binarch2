use {
    crate::magic::MAX_MAGIC_LEN,
    anyhow::{anyhow, Result},
    indicatif::{ParallelProgressIterator, ProgressBar},
    rayon::iter::{IntoParallelIterator, ParallelIterator},
    std::{cmp::Reverse, fmt},
};

use {
    crate::magic::{magics, Kind},
    std::collections::{hash_map::Iter, HashMap, HashSet},
};

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

    pub fn new_parallel(
        data: &[u8],
        num_chunks: usize,
        progress_bar: ProgressBar,
    ) -> Result<Binarch> {
        let len = data.len();
        let chunk_len = usize::max(MAX_MAGIC_LEN, len / num_chunks);

        let chunks = (0..len)
            .step_by(chunk_len)
            .map(|x| {
                let limit = usize::min(x + chunk_len + MAX_MAGIC_LEN, len);
                data.get(x..limit).map(|d| (x, d))
            })
            .collect::<Option<Vec<(usize, &[u8])>>>()
            .ok_or_else(|| anyhow!("Failed to read chunks"))?;

        progress_bar.set_length(chunks.len() as u64);

        let results = chunks
            .into_par_iter()
            .progress_with(progress_bar)
            .map(|(i, ck)| Binarch::new(i, ck))
            .reduce(Binarch::default, Binarch::reduce);
        Ok(results)
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

    pub fn result(&self) -> Option<Kind> {
        self.into_iter()
            .max_by_key(|(_, v)| v.len())
            .map(|(k, _)| (k.clone()))
    }
}

impl fmt::Debug for Binarch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut list = self
            .into_iter()
            .map(|(k, v)| (k, v.len()))
            .collect::<Vec<(&Kind, usize)>>();
        list.sort_by_key(|(_, v)| Reverse(*v));

        writeln!(fmt, "Binarch:")?;
        for (k, v) in list {
            writeln!(fmt, "\t{:#?}: {}", k, v)?;
        }
        Ok(())
    }
}
