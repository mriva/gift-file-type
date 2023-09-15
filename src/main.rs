use clap::Parser;

mod cli_commands;
mod gift_to_excel;

fn main() -> anyhow::Result<()> {
    let args = cli_commands::Args::parse();

    match args.direction {
        cli_commands::Direction::GiftToCsv => {
            println!("Gift to Csv");
            gift_to_excel::convert(&args.input, &args.output)?;
        }
        cli_commands::Direction::CsvToGift => {
            println!("Csv to Gift");
        }
    }

    Ok(())
}
