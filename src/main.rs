use clap::Parser;

mod cli_commands;
mod gift_to_excel;

fn main() -> anyhow::Result<()> {
    let args = cli_commands::Args::parse();

    match args.direction {
        cli_commands::Direction::GiftToExcel => {
            println!("Gift to Excel");
            //gift_to_excel::convert();
        }
        cli_commands::Direction::ExcelToGift => {
            println!("Excel to Gift");
        }
    }

    Ok(())
}
