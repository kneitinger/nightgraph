use clap::{crate_authors, crate_description, crate_version, AppSettings, Clap};
use nightgraphics::render::SvgRenderer;
use serde::{Deserialize, Serialize};
use sketches::SketchList;

#[derive(Clap, Serialize, Deserialize)]
#[clap(about= crate_description!(), version = crate_version!(), author = crate_authors!())]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    sketch: SketchList,
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
