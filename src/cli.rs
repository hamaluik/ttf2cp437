use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("FONT")
                .required(true)
                .index(1)
                .help("the path to a .ttf font file to process"),
        )
        .arg(
            Arg::with_name("HEIGHT")
                .required(true)
                .index(2)
                .help("the height (in px) for each glyph"),
        )
        .arg(
            Arg::with_name("SCALE")
                .help("an optional real number scale to apply to the resulting glyph set"),
        )
}
