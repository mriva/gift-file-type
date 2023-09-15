use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output: String,

    #[arg(short, long)]
    pub direction: Direction,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum Direction {
    GiftToExcel,
    ExcelToGift,
}
