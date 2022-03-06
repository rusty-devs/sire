use clap::Parser;
mod lib;
/// Project generation tool for managing template projects
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the folder containing the template files to process
    #[clap(short, long)]
    source_path: String,
}

fn main() {
    let args = Args::parse();

    lib::hello_world();
    println!("Processing template {}", args.source_path);
}
