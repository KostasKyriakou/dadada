
use clap::{Arg, App};
use std::fs;
use std::path::Path;

use dadada::{Block, Options, extract, build_html};

fn main() {
    let matches = App::new("dadada")
        .version("0.9.2")
        .author("Benjamin Kampmann <ben@gnunicorn.org>, Rui Vieira <ruidevieira@googlemail.com>")
        .about("Artisanal Rust inlined code documentation renderer")

        .arg(Arg::with_name("title")
            .short("t")
            .long("title")
            .value_name("String")
            .help("The HTML title to render")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("target file to render to, stdout if not given")
            .takes_value(true))

        .arg(Arg::with_name("input")
            .value_name("FILE")
            .help("rust source files")
            .required(true)
            .multiple(true)
            .takes_value(true))
        .get_matches();

    let output = build_html(
        matches.values_of("input").expect("This is required")
            .map(|i| {
                let mut blocks = extract(i.to_string());
                let path = Path::new(i);
                let title = path.file_name().expect("Must be a file").to_str().unwrap_or("");
                blocks.insert(0, Block::new_file(title, i));
                blocks
            }).flatten(),
        Options {
            title: matches.value_of("title").unwrap_or("").to_string(),
            with_css: true,
        },
    );

    match matches.value_of("output") {
        Some(f) => fs::write(f, output).expect("Could not write to output file."),
        None => println!("{}",  output),
    }
}

