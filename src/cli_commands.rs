use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// input filename
    #[arg(short, long)]
    pub input: String,

    /// output filename
    #[arg(short, long)]
    pub output: String,

    /// conversion direction
    #[arg(short, long)]
    pub direction: Direction,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Direction {
    GiftToCsv,
    CsvToGift,
}
