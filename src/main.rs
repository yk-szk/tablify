use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets the input csv/xlsx file
    input: String,

    /// Sets the template file
    template: String,

    /// Input data has header line
    #[clap(long)]
    header: bool,

    /// Enable autoescaping
    #[clap(short = 'e', long)]
    escape: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    let raw_content = fs::read(&args.input)?;
    let template_content = fs::read_to_string(args.template)?;
    let html = tablify::tablify(
        &template_content,
        &raw_content,
        args.input.as_str(),
        args.header,
        args.escape,
    )?;
    println!("{}", html);

    Ok(())
}
