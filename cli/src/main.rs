use clap::{crate_authors, crate_description, crate_version, Parser};
use nightgraphics::render::SvgRenderer;
use nightsketch::SketchSubcommand;
use serde::{Deserialize, Serialize};

#[derive(clap::Parser, Serialize, Deserialize)]
#[clap(about= crate_description!(), version = crate_version!(), author = crate_authors!())]
struct Opts {
    #[clap(subcommand)]
    sketch: SketchSubcommand,

    /// Path where the resulting SVG file is stored
    #[clap(long, default_value = "drawing.svg")]
    output: String,
}

fn main() {
    let opts = Opts::parse();
    let opts_json = serde_json::to_string(&opts).unwrap();
    // JSON serialization will eventually be used for config file
    // saving and loading
    println!("{}", opts_json);
    let canvas = opts.sketch.exec();
    match canvas {
        Ok(c) => c.render_svg(&opts.output),
        Err(e) => println!("Error rendering sketch: {:?}", e),
    }
}
