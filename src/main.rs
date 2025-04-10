use argh::FromArgs;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::path::PathBuf;
use std::sync::LazyLock;

/// Strips ANSI colour codes from piped input or files printing out or saving to file.
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
struct Args {
    /// input file - if not provided will read from stdin
    #[argh(option, short = 'i')]
    input: Option<PathBuf>,
    /// output file - if not provided will print to stdout
    #[argh(option, short = 'o')]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let args: Args = argh::from_env();

    let reader = if let Some(input) = args.input {
        let f = File::open(input)?;
        BufReader::new(Box::new(f) as Box<dyn Read>)
    } else {
        BufReader::new(Box::new(io::stdin()) as Box<dyn Read>)
    };

    let mut out_writer = match args.output {
        Some(x) => {
            let file = File::create(x).unwrap();
            BufWriter::new(Box::new(file) as Box<dyn Write>)
        }
        None => BufWriter::new(Box::new(io::stdout()) as Box<dyn Write>),
    };

    for line in reader.lines().filter_map(Result::ok).map(replace) {
        out_writer.write_all(line.as_bytes())?;
        out_writer.write_all(b"\n")?;
    }

    Ok(())
}

pub static COLOUR_CODES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\x1B\[[0-9;]*m"#).unwrap());

fn replace<'a>(line: String) -> String {
    (&*COLOUR_CODES).replace_all(&line, "").to_string()
}
