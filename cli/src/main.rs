use clap::{crate_authors, crate_description, crate_version, Parser};
use nightgraphics::render::SvgRenderer;
use nightsketch::SketchSubcommand;
use serde::{Deserialize, Serialize};

#[derive(clap::Parser, Serialize, Deserialize)]
#[clap(about= crate_description!(), version = crate_version!(), author = crate_authors!())]
struct Opts {
    #[clap(subcommand)]
    sketch: SketchSubcommand,
}

fn main() {
    let opts = Opts::parse();
    let opts_json = serde_json::to_string(&opts).unwrap();
    // JSON serialization will eventually be used for config file
    // saving and loading
    println!("{}", opts_json);
    let canvas = opts.sketch.exec().unwrap();
    canvas.render_svg();
}
