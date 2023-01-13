use crate::{arg::Opt, magic::Kind};
use {
    anyhow::{anyhow, Result},
    clap::Parser,
    log::{debug, info},
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
    std::{
        cmp::Reverse,
        collections::{HashMap, HashSet},
        fs::OpenOptions,
    },
};

const NUM_CHUNKS: usize = 4096;
const OVERLAP_SIZE: usize = 16;

mod arg;
mod magic;

fn main() -> Result<()> {
    let opt = Opt::parse();

    env_logger::builder()
        .filter_level(opt.log_level.into())
        .format_timestamp(None)
        .format_target(false)
        .init();

    let f = OpenOptions::new().read(true).open(opt.input)?;
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

    let mut list = matches
        .iter()
        .map(|(k, v)| (k, v.len()))
        .collect::<Vec<(&Kind, usize)>>();
    list.sort_by_key(|(_, v)| Reverse(*v));

    debug!("Results:");
    for (k, v) in list {
        debug!("\t{:#?}: {}", k, v);
    }

    match matches.iter().max_by_key(|(_, v)| v.len()) {
        Some((k, _)) => info!("{:#?}", k),
        None => {
            info!("Unknown")
        }
    }

    info!("DONE");
    Ok(())
}
