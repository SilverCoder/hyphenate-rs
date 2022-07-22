use std::io::{self, Write};

use clap::Parser;
use hyphenation::{Hyphenator, Language, Load, Standard};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "en")]
    language: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(language) = Language::try_from_code(args.language) {
        if let Ok(dictionary) = Standard::from_embedded(language) {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;

            let hyphenated = dictionary.hyphenate(&buffer);
            let mut hyphenation_iter = hyphenated.into_iter();
            hyphenation_iter.mark_with("\u{00ad}");

            let processed_string = hyphenation_iter.collect::<Vec<String>>().join("");

            io::stdout().write_all(&processed_string.as_bytes())?;
        }
    }

    Ok(())
}
