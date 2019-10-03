use super::interface::{Crates, Display, Network};

pub struct NetworkSimulator {
    result: Vec<String>,
    err: String,
    should_fail: bool,
}

impl NetworkSimulator {
    pub fn new(result: Vec<String>, err: String, should_fail: bool) -> Self {
        let network_simulator = NetworkSimulator {
            result,
            err,
            should_fail,
        };
        network_simulator
    }
}

impl Network for NetworkSimulator {
    fn get_crate_version(&self, _: String) -> Result<Vec<String>, String> {
        if self.should_fail {
            return Err(self.err.clone());
        }
        Ok(self.result.clone())
    }
}

pub struct CratesSimulator {
    result: Vec<String>,
    err: String,
    should_fail_read: bool,
    should_fail_write: bool,
}

impl CratesSimulator {
    pub fn new(
        result: Vec<String>,
        err: String,
        should_fail_read: bool,
        should_fail_write: bool,
    ) -> Self {
        let crates_simulator = CratesSimulator {
            result,
            err,
            should_fail_read,
            should_fail_write,
        };
        crates_simulator
    }
}

impl Crates for CratesSimulator {
    fn read(&self) -> Result<Vec<String>, String> {
        if self.should_fail_read {
            return Err(self.err.clone());
        }
        Ok(self.result.clone())
    }

    fn write(&self, content: Vec<String>) -> Result<(), String> {
        if self.should_fail_write {
            return Err(self.err.clone());
        }
        Ok(())
    }
}

pub struct DisplaySimulator {}

impl DisplaySimulator {
    pub fn new() -> DisplaySimulator {
        DisplaySimulator {}
    }
}

impl Display for DisplaySimulator {
    fn start_loading(&mut self, message: &str) {}
    fn stop_loading(&mut self, success: bool) {}
}
