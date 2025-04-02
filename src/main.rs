use argh::FromArgs;
use std::borrow::Cow;
use regex::Regex;
use std::sync::LazyLock;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(FromArgs)]
/// Whatever
struct Args {
    /// input file
    #[argh(option)]
    input: Option<PathBuf>,
    /// output file
    #[argh(option)]
    output: Option<PathBuf>,
}


fn main() -> std::io::Result<()>{
    let args: Args = argh::from_env();
    
    if let Some(input) = args.input {
        let f = File::open(input)?;
        let mut reader = BufReader::new(f);

        let mut output_file = args.output.map(|x| {
            let mut file = File::create(x).unwrap();
            file
        });

        for line in reader.lines().filter_map(Result::ok).map(replace) {
            if let Some(f) = &mut output_file {
                f.write_all(line.as_bytes())?;
                f.write_all(b"\n")?;
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

static COLOUR_CODES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\x1B\[[0-9;]*m"#).unwrap()
});

fn replace<'a>(line: String) -> String {
    (&*COLOUR_CODES).replace_all(&line, "").to_string()
}
