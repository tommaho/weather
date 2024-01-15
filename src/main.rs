use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]


struct Args {
    /// Input text
    #[arg(required(true))]
    zip: Vec<String>,

    /// More verbose output?
    #[arg(short('n'))]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    print!("{}", args.zip.join(" "));
}