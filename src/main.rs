use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;
use toml;

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    for timeline_path in opt.timeline_paths.iter() {
        let mut fh = File::open(timeline_path)?;
        let mut payload = String::new();
        fh.read_to_string(&mut payload)?;
        let timeline: TimelineSpec = toml::from_str(&payload)?;
        println!("{:#?}", timeline);
    }
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rss-reader")]
struct Opt {
    /// Path(s) to your timeline TOML file(s)
    #[structopt(name = "FILE", parse(from_os_str))]
    timeline_paths: Vec<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct TimelineSpec {
    meta: TimelineSpecMeta,
    content: TimelineSpecContent,
}

#[derive(Debug, Deserialize)]
struct TimelineSpecMeta {
    name: String,
}

#[derive(Debug, Deserialize)]
struct TimelineSpecContent {
    feeds: Vec<FeedSpec>,
}

#[derive(Debug, Deserialize)]
struct FeedSpec {
    name: String,
    uri: String,
    poll_freq_seconds: Option<u32>,
}
