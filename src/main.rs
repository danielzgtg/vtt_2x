use clap::clap_app;

use vtt_2x::run;

fn main() {
    let matches = clap_app!((env!("CARGO_PKG_NAME")) =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@arg PATH: +required #{1,9} "The input files")
    )
    .get_matches();
    run(matches.values_of("PATH").unwrap());
}
