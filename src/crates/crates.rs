use std::borrow::BorrowMut;
use std::io::{BufRead, BufReader};

use super::super::add::interface;
use super::helper::HelperInterface;

const CARGO_PATH: &str = "Cargo.toml";

pub struct Crates {
    helper: Box<dyn HelperInterface>,
}

impl Crates {
    pub fn new(helper: Box<dyn HelperInterface>) -> Self {
        let crates = Crates { helper };
        crates
    }
}

impl interface::Crates for Crates {
    fn read(&self) -> Result<Vec<String>, String> {
        let file = self.helper.open(CARGO_PATH)?;
        let buffered = BufReader::new(file);

        let mut result: Vec<String> = Vec::new();
        for line in buffered.lines() {
            result.push(self.helper.read(line)?);
        }

        Ok(result)
    }

    fn write(&self, content: Vec<String>) -> Result<(), String> {
        let mut file = self.helper.open_or_create(CARGO_PATH)?;
        for data in content {
            self.helper.write(file.borrow_mut(), data)?;
        }
        Ok(())
    }
}
