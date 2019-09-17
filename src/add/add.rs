use super::interface::{Crates, Network};

pub struct Add {
    network: Box<dyn Network>,
    crates: Box<dyn Crates>,
    name: String,
}

impl Add {
    fn new(name: String, network: Box<dyn Network>, crates: Box<dyn Crates>) -> Self {
        let add = Add {
            name,
            network,
            crates,
        };
        add
    }

    fn add_crates_dependency(&self) -> Result<(), String> {
        let version = self.network.get_crate_version(self.name.clone())?;
        self.crates.add_dependency(self.name.clone(), version)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::super::simulator::{CratesSimulator, NetworkSimulator};
    use super::*;

    #[test]
    fn handle_network_error() {
        let error_message = String::from("mock error");
        let network = NetworkSimulator::new(String::from(""), error_message.clone(), true);
        let crates = CratesSimulator::new(error_message.clone(), true);

        let add = Add::new(String::from(""), Box::new(network), Box::new(crates));
        let result = add.add_crates_dependency();
        assert!(result.is_err())
    }

    #[test]
    fn handle_crates_error() {
        let error_message = String::from("mock error");
        let network = NetworkSimulator::new(String::from("1.0.0"), error_message.clone(), false);
        let crates = CratesSimulator::new(error_message.clone(), true);

        let add = Add::new(String::from(""), Box::new(network), Box::new(crates));
        let result = add.add_crates_dependency();
        assert!(result.is_err())
    }
}
