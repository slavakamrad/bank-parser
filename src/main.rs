use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io;
use transact_parser::{parse_format, write_format};

#[derive(Parser, Debug)]
#[command(name = "transact_converter")]
#[command(about = "Конвертер финансовых данных между форматами")]
#[command(version = "1.0")]
struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(long = "input-format", value_enum)]
    input_format: Format,

    #[arg(long = "output-format", value_enum)]
    output_format: Format,

    #[arg(short, long)]
    output: Option<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Csv,
    Binary,
    Text,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Csv => write!(f, "csv"),
            Format::Binary => write!(f, "binary"),
            Format::Text => write!(f, "text"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let input: Box<dyn io::Read> = if cli.input == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(&cli.input)?)
    };

    let transactions = parse_format(input, &cli.input_format.to_string())?;

    let output: Box<dyn io::Write> = match cli.output {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };

    write_format(&transactions, output, &cli.output_format.to_string())?;

    Ok(())
}
