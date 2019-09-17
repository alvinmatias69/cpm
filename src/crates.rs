use super::add::interface;

pub struct Crates {}

impl interface::Crates for Crates {
    fn add_dependency(&self, name: String, version: String) -> Result<(), String> {
        unimplemented!();
    }
}
