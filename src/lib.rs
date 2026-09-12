mod history;
mod unit;

use anyhow::{Context, Result};
use history::HistoryList;
use unit::{Unit, UnitCategory, length_conversion, temperature_conversion};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "unitconv",
    version = "1.0",
    about = "Aplikasi All In One Converter"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Konversi satuan ke berbagai satuan lainnya
    Convert {
        /// Define origin unit
        #[arg(long)]
        from: String,

        /// Define destination unit
        #[arg(long)]
        to: String,

        /// Define value to convert
        #[arg(long)]
        value: f64,
    },
    /// Mencetak satuan yang didukung
    List,
    /// Mencetak log history perintah
    History,
}

pub fn run(cli: Cli) -> Result<()> {
    let mut histories = HistoryList::load().unwrap_or_default();
    let mut updated = false;

    match cli.command {
        Commands::Convert { from, to, value } => {
            let from_unit = Unit::from_str(&from);
            let to_unit = Unit::from_str(&to);

            match (from_unit, to_unit) {
                (None, _) => println!("Error: [ERROR] Satuan asal '{}' tidak dikenali.", from),
                (_, None) => println!("Error: [ERROR] Satuan tujuan '{}' tidak dikenali.", to),
                (Some(f), Some(t)) if f.category() == t.category() => match f.category() {
                    UnitCategory::Temperature => {
                        let result = temperature_conversion(f, t, value);
                        histories.add(result);
                        updated = true;
                    }
                    UnitCategory::Length => {
                        let result = length_conversion(f, t, value);
                        histories.add(result);
                        updated = true;
                    }
                },
                (Some(f), Some(t)) => {
                    panic!(
                        "Error: [ERROR] Tidak dapat mengonversi satuan yang berbeda kategori: [panjang] {} → [suhu] {}",
                        f.name(),
                        t.name()
                    )
                }
            }

            if updated {
                histories.save().context("Failed to log history.")?;
            }
        }
        Commands::List => {
            println!("Satuan yang didukung:");
            println!("1. [suhu] celsius");
            println!("2. [suhu] fahrenheit");
            println!("3. [suhu] kelvin");
            println!("4. [panjang] cm");
            println!("5. [panjang] inch");
            println!("6. [panjang] km");
            println!("7. [panjang] miles");
        }
        Commands::History => {
            histories.print();
        }
    }

    Ok(())
}
