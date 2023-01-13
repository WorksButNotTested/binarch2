use crate::{arg::Opt, binarch::Binarch, magic::Kind};
use {
    anyhow::{anyhow, Result},
    clap::Parser,
    indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle},
    log::{debug, info},
    rayon::iter::{IntoParallelIterator, ParallelIterator},
    std::{cmp::Reverse, fs::OpenOptions},
};

const NUM_CHUNKS: usize = 4096;
const OVERLAP_SIZE: usize = 16;

mod arg;
mod binarch;
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

    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(ProgressStyle::default_bar()
    .template(
        "{spinner:.green} [{elapsed_precise:.green}] [{eta_precise:.cyan}] {msg:.magenta} ({percent:.bold}%) [{bar:30.cyan/blue}]",
    )?
    .progress_chars("█░"));
    progress_bar.set_length(chunks.len() as u64);

    let matches = chunks
        .into_par_iter()
        .progress_with(progress_bar)
        .map(|(i, ck)| Binarch::new(i, ck));
    let results = matches.reduce(Binarch::default, Binarch::reduce);

    let mut list = results
        .into_iter()
        .map(|(k, v)| (k, v.len()))
        .collect::<Vec<(&Kind, usize)>>();
    list.sort_by_key(|(_, v)| Reverse(*v));

    debug!("Results:");
    for (k, v) in list {
        debug!("\t{:#?}: {}", k, v);
    }

    match results.into_iter().max_by_key(|(_, v)| v.len()) {
        Some((k, _)) => info!("{:#?}", k),
        None => {
            info!("Unknown")
        }
    }

    info!("DONE");
    Ok(())
}
