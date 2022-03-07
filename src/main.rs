use clap::StructOpt;
use sire::{App, Config};

fn main() {
    let conf = Config::parse();
    let app = App::from(conf);

    app.run();
}
