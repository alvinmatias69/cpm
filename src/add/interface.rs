pub trait Network {
    fn get_crate_version(&self, name: String) -> Result<Vec<String>, String>;
}

pub trait Crates {
    fn read(&self) -> Result<Vec<String>, String>;
    fn write(&self, content: Vec<String>) -> Result<(), String>;
}

pub trait Display {
    fn start_loading(&mut self, message: &str);
    fn stop_loading(&mut self);
    fn success(&self, message: String);
    fn failure(&self, message: String);
}
