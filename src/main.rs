use clap::Parser;
use mokaccino::client::client;

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
    client::start();
}
