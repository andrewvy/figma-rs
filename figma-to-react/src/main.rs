use async_std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    token: Option<String>,

    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let contents = fs::read(opt.file).await?;
    let contents = String::from_utf8(contents).expect("Invalid UTF-8");

    let document: figma_api::file::File = serde_json::from_str(&contents)?;

    println!("{:?}", document);

    Ok(())
}
