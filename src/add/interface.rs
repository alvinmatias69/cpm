pub trait Network {
    fn get_crate_version(&self, name: String) -> Result<String, String>;
}

pub trait Crates {
    fn add_dependency(&self, name: String, version: String) -> Result<(), String>;
}
