// Basic weights calculation
mod corpus;
// Layout generation
mod generation;
// Parser for precalculated weights
mod parser;

use std::error::Error;

pub struct Args {
    cutoff: f64,
    alphabet: Option<[char; 30]>,
    file: Option<String>,
    preset_left: Option<[char; 3]>,
    preset_right: Option<[char; 9]>,
    precalculated: bool,
}

const HELP: &str = r#"unknown option. Valid options:
--cutoff [cutoff]     - specifies score cutoff for generated layouts. Must be in range [0.0, 1.0).
--precalculated       - indicates that weights are supplied precalculated and stored in json format.
--file [path]         - indicates to get weights from a file instead of stdio.
--preset-left [keys]  - fixes the displayed leftmost column. Provide 3 characters in top-to-bottom order; whitespace and `|` separators are ignored.
--preset-right [keys] - fixes the displayed right three columns. Provide 9 characters in row-major order; whitespace and `|` separators are ignored.
--alphabet [alphabet] - sets or overrides alphabet used for optimization. Alphabet must be quoted and with special characters (quote and backslash) escaped"#;

fn parse_alphabet(str: &str) -> Result<[char; 30], Box<dyn Error>> {
    str.chars()
        .collect::<Vec<char>>()
        .try_into()
        .map_err(|v: Vec<_>| format!("alphabet should have 30 characters, not {}", v.len()).into())
}

fn parse_preset_right(str: &str) -> Result<[char; 9], Box<dyn Error>> {
    str.chars()
        .filter(|c| !c.is_whitespace() && *c != '|')
        .collect::<Vec<char>>()
        .try_into()
        .map_err(|v: Vec<_>| format!("preset-right should have 9 characters after removing whitespace and `|`, not {}", v.len()).into())
}

fn parse_preset_left(str: &str) -> Result<[char; 3], Box<dyn Error>> {
    str.chars()
        .filter(|c| !c.is_whitespace() && *c != '|')
        .collect::<Vec<char>>()
        .try_into()
        .map_err(|v: Vec<_>| format!("preset-left should have 3 characters after removing whitespace and `|`, not {}", v.len()).into())
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let mut alphabet: Option<[char; 30]> = None;
    let mut file = None;
    let mut preset_left = None;
    let mut preset_right = None;
    let mut precalculated = false;
    let mut cutoff = 1.0;

    let mut iter = std::env::args().skip(1);
    while let Some(option) = iter.next() {
        match option.as_str() {
            "--cutoff" => cutoff = iter.next().unwrap_or("1.0".to_owned()).parse::<f64>()?,
            "--precalculated" => precalculated = true,
            "--file" => file = Some(iter.next().unwrap_or("weights.json".to_owned())),
            "--alphabet" => alphabet = Some(parse_alphabet(&iter.next().unwrap_or_default())?),
            "--preset-left" => preset_left = Some(parse_preset_left(&iter.next().unwrap_or_default())?),
            "--preset-right" => preset_right = Some(parse_preset_right(&iter.next().unwrap_or_default())?),
            _ => return Err(HELP.into()),
        }
    }
    if !(0.0..1.0).contains(&cutoff) {
        return Err("cutoff must be defined and in range [0.0, 1.0)".into());
    }
    Ok(Args {
        cutoff,
        alphabet,
        file,
        preset_left,
        preset_right,
        precalculated,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    if args.alphabet.is_none() && !args.precalculated {
        return Err("missing alphabet".into());
    }
    let weights = &match args.file {
        None => std::io::read_to_string(std::io::stdin())?,
        Some(file) => std::fs::read_to_string(file)?,
    };
    let (alphabet, weights) = match args.precalculated {
        true => parser::parse(weights).map_err(|e| e.to_string())?,
        false => (None, corpus::weights(weights)),
    };
    let alphabet = args.alphabet.or(alphabet).ok_or("missing alphabet")?;
    println!("weights loaded");
    for (layout, score) in generation::generator(
        alphabet,
        weights,
        args.cutoff,
        args.preset_left,
        args.preset_right,
    )? {
        println!("found layout with score {score}");
        for (i, c) in layout.into_iter().enumerate() {
            print!("{c}{}", match i % 10 {
                9 => '\n',
                _ => ' ',
            });
        }
    }
    println!("finished");
    Ok(())
}
