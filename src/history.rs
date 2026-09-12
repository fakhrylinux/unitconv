use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct History {
    log: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct HistoryList {
    pub histories: Vec<History>,
}

impl HistoryList {
    const FILE_PATH: &'static str = "conversion.json";

    pub fn load() -> Result<Self> {
        if !Path::new(Self::FILE_PATH).exists() {
            return Ok(Self::default());
        }

        let data = fs::read_to_string(Self::FILE_PATH)?;
        let list = serde_json::from_str(&data)?;
        Ok(list)
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }

    pub fn add(&mut self, log: String) {
        self.histories.push(History { log: log });
    }

    pub fn print(&self) {
        println!("Riwayat Konversi:");
        for (i, log) in self.histories.iter().enumerate() {
            println!("{}. {}", i + 1, log.log);
        }
    }
}
