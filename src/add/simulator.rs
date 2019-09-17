use super::interface::{Crates, Network};

pub struct NetworkSimulator {
    result: String,
    err: String,
    should_fail: bool,
}

impl NetworkSimulator {
    pub fn new(result: String, err: String, should_fail: bool) -> Self {
        let network_simulator = NetworkSimulator {
            result,
            err,
            should_fail,
        };
        network_simulator
    }
}

impl Network for NetworkSimulator {
    fn get_crate_version(&self, _: String) -> Result<String, String> {
        if self.should_fail {
            return Err(self.err.clone());
        }
        Ok(self.result.clone())
    }
}

pub struct CratesSimulator {
    err: String,
    should_fail: bool,
}

impl CratesSimulator {
    pub fn new(err: String, should_fail: bool) -> Self {
        let crates_simulator = CratesSimulator { err, should_fail };
        crates_simulator
    }
}

impl Crates for CratesSimulator {
    fn add_dependency(&self, _: String, _: String) -> Result<(), String> {
        if self.should_fail {
            return Err(self.err.clone());
        }
        Ok(())
    }
}
