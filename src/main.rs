use crate::magic::Kind;
use {
    anyhow::{anyhow, Result},
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
    std::{
        collections::{HashMap, HashSet},
        env,
        fs::OpenOptions,
    },
};

const NUM_CHUNKS: usize = 4096;
const OVERLAP_SIZE: usize = 16;

mod magic;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = args
        .get(1)
        .ok_or_else(|| anyhow!("No input file provided"))?;

    let f = OpenOptions::new().read(true).open(input)?;
    let data = unsafe { memmap::MmapOptions::new().map(&f)? };
    let len = f.metadata()?.len() as usize;
    let chunk_len = usize::max(1, len / NUM_CHUNKS);

    let chunks = (0..len)
        .step_by(chunk_len)
        .map(|x| {
            let limit = usize::min(x + chunk_len + OVERLAP_SIZE, len);
            data.get(x..limit).map(|d| (x, d))
        })
        .collect::<Option<Vec<(usize, &[u8])>>>()
        .ok_or_else(|| anyhow!("Failed to read chunks"))?;

    let matches = chunks
        .par_iter()
        .map(|(i, ck)| {
            let mut h = HashMap::<Kind, HashSet<usize>>::new();
            for m in magic::magics().iter() {
                let indexes = m
                    .regex()
                    .find_iter(ck)
                    .filter(|c| m.matches(&ck[c.start()..c.end()]))
                    .map(|c| i + c.start())
                    .collect::<HashSet<usize>>();
                match h.get_mut(&m.kind()) {
                    Some(hs) => hs.extend(indexes),
                    None => {
                        h.insert(m.kind(), indexes);
                    }
                }
            }
            h
        })
        .reduce(HashMap::<Kind, HashSet<usize>>::new, |mut a, b| {
            for (bk, bv) in b {
                match a.get_mut(&bk) {
                    Some(av) => av.extend(bv.iter()),
                    None => {
                        a.insert(bk, bv);
                    }
                }
            }
            a
        });

    for (k, v) in matches {
        println!("{:#?}: {}", k, v.len());
    }

    println!("DONE");
    Ok(())
}
