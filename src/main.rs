use std::env;

use clap::StructOpt;
use sire::{App, Config};

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let conf = Config::parse();
    let app = App::from(conf);

    app.run().ok();
}
