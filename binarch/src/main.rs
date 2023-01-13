use {
    crate::arg::Opt,
    anyhow::Result,
    clap::Parser,
    indicatif::{ProgressBar, ProgressStyle},
    log::{debug, info},
    rbinarch::Binarch,
    std::fs::OpenOptions,
};

const NUM_CHUNKS: usize = 4096;

mod arg;

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

    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(ProgressStyle::default_bar()
    .template(
        "{spinner:.green} [{elapsed_precise:.green}] [{eta_precise:.cyan}] {msg:.magenta} ({percent:.bold}%) [{bar:80.cyan/blue}]",
    )?);

    let binarch = Binarch::new_parallel(&data[..len], NUM_CHUNKS, progress_bar)?;
    debug!("{binarch:#?}");

    match binarch.result() {
        Some(k) => info!("{:#?}", k),
        None => info!("Unknown"),
    }

    info!("DONE");
    Ok(())
}
