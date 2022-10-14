use clap::Parser;
use std::io::Read;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub output: Option<String>,
    #[clap(short, long, value_parser)]
    pub input: Option<String>,
    #[clap(short, long)]
    pub encode: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut input = String::new();
    if let Some(ref input_path) = args.input {
        input = std::fs::read_to_string(input_path)?;
    } else {
        std::io::stdin().read_to_string(&mut input)?;
    }
    if args.encode {
        let ajson = serde_json::from_str::<agprefs::Agpref>(&input);
    } else {
        let agprefs = agprefs::Agpref::from_str(&input)?;
        let ajson = serde_json::to_string_pretty(&agprefs)?;
        if let Some(ref output) = args.output {
            std::fs::write(output, ajson)?;
        } else {
            println!("{}", ajson);
        }
    }
    Ok(())
}
