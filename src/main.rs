use mokaccino::client::client::Client;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    mode: String,
    port: u16,

}

fn main() {
    //let args = Args::parse();
    //println!("{:?}", args);

}
