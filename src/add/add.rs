use super::interface::{Crates, Display, Network};

pub struct Add {
    network: Box<dyn Network>,
    crates: Box<dyn Crates>,
    display: Box<dyn Display>,
    name: String,
    version: String,
}

const LATEST: &'static str = "latest";
const DEPENDENCIES_KEY: &'static str = "[dependencies]";

impl Add {
    pub fn new(
        name: String,
        version: String,
        network: Box<dyn Network>,
        crates: Box<dyn Crates>,
        display: Box<dyn Display>,
    ) -> Self {
        let add = Add {
            name,
            version,
            network,
            crates,
            display,
        };
        add
    }

    pub fn crates_dependency(&mut self) -> Result<(), String> {
        self.display.start_loading("Reading Cargo.toml");
        let cargo = self.crates.read()?;
        self.display.stop_loading();

        self.display.start_loading("Requesting to crates.io");
        let versions = self.network.get_crate_version(self.name.clone())?;
        self.set_required_version(&versions)?;
        self.display.stop_loading();

        self.display.start_loading("Writing Cargo.toml");
        let cargo_result = self.add_dependency_to_crate(&cargo)?;
        self.crates.write(cargo_result)?;
        self.display.stop_loading();

        Ok(())
    }

    fn set_required_version(&mut self, versions: &Vec<String>) -> Result<(), String> {
        if self.version == String::from(LATEST) {
            self.version = versions[0].clone();
            return Ok(());
        }

        for item in versions {
            if self.version == *item {
                return Ok(());
            }
        }

        Err(String::from("version unavailable"))
    }

    fn add_dependency_to_crate(&self, cargo: &Vec<String>) -> Result<Vec<String>, String> {
        let mut result: Vec<String> = Vec::new();
        let dep_key = String::from(DEPENDENCIES_KEY);
        let mut dep_key_found: bool = false;

        for item in cargo {
            result.push(item.clone());
            if !dep_key_found && *item == dep_key {
                result.push(self.create_version_string());
                dep_key_found = true;
            }
        }

        if !dep_key_found {
            result.push(dep_key);
            result.push(self.create_version_string());
        }

        Ok(result)
    }

    fn create_version_string(&self) -> String {
        let mut result = self.name.clone();
        result.push_str(" = \"");
        result.push_str(&self.version);
        result.push_str("\"");
        result
    }
}

// #[cfg(test)]
// mod test {
//     use super::super::simulator::{CratesSimulator, NetworkSimulator};
//     use super::*;

//     #[test]
//     fn handle_crate_read_error() {
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(vec![String::from("")], error_message.clone(), false);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let mut add = Add::new(
//             String::from(""),
//             String::from(""),
//             Box::new(network),
//             Box::new(crates),
//         );
//         let result = add.crates_dependency();
//         assert!(result.is_err())
//     }

//     #[test]
//     fn handle_network_error() {
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(vec![String::from("")], error_message.clone(), true);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let mut add = Add::new(
//             String::from(""),
//             String::from(""),
//             Box::new(network),
//             Box::new(crates),
//         );
//         let result = add.crates_dependency();
//         assert!(result.is_err())
//     }

//     #[test]
//     fn get_latest_version() {
//         let versions = vec![String::from("2.0.0"), String::from("1.0.0")];
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(versions.clone(), error_message.clone(), true);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let mut add = Add::new(
//             String::from(""),
//             String::from(LATEST),
//             Box::new(network),
//             Box::new(crates),
//         );

//         add.set_required_version(&versions).unwrap();
//         assert_eq!(add.version, versions[0]);
//     }

//     #[test]
//     fn get_desired_version() {
//         let versions = vec![String::from("2.0.0"), String::from("1.0.0")];
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(versions.clone(), error_message.clone(), true);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let mut add = Add::new(
//             String::from(""),
//             versions[1].clone(),
//             Box::new(network),
//             Box::new(crates),
//         );

//         add.set_required_version(&versions).unwrap();
//         assert_eq!(add.version, versions[1]);
//     }

//     #[test]
//     fn error_on_unavailable_version() {
//         let versions = vec![String::from("2.0.0"), String::from("1.0.0")];
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(versions.clone(), error_message.clone(), true);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let mut add = Add::new(
//             String::from(""),
//             String::from("3.0.0"),
//             Box::new(network),
//             Box::new(crates),
//         );

//         let result = add.set_required_version(&versions);
//         assert!(result.is_err())
//     }

//     #[test]
//     fn generate_version_string() {
//         let versions = vec![String::from("2.0.0"), String::from("1.0.0")];
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(versions.clone(), error_message.clone(), true);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let add = Add::new(
//             String::from("test-dep"),
//             String::from("2.0.0"),
//             Box::new(network),
//             Box::new(crates),
//         );

//         let result = add.create_version_string();
//         assert_eq!(result, String::from("test-dep = \"2.0.0\""));
//     }

//     #[test]
//     fn add_cargo_version_after_dep_tag() {
//         let versions = vec![String::from("2.0.0"), String::from("1.0.0")];
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(versions.clone(), error_message.clone(), true);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let name = String::from("dep-test");
//         let version = String::from("1.0.0");

//         let add = Add::new(
//             name.clone(),
//             version.clone(),
//             Box::new(network),
//             Box::new(crates),
//         );

//         let cargo = vec![String::from("name = test"), String::from(DEPENDENCIES_KEY)];

//         let result = add.add_dependency_to_crate(&cargo).unwrap();
//         assert_eq!(cargo.len() + 1, result.len());

//         let mut name_version = name.clone();
//         name_version.push_str(" = \"");
//         name_version.push_str(&version);
//         name_version.push_str("\"");
//         assert_eq!(result[result.len() - 1], name_version);
//     }

//     #[test]
//     fn add_cargo_version_and_dep_tag() {
//         let versions = vec![String::from("2.0.0"), String::from("1.0.0")];
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(versions.clone(), error_message.clone(), true);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), true, true);

//         let name = String::from("dep-test");
//         let version = String::from("1.0.0");

//         let add = Add::new(
//             name.clone(),
//             version.clone(),
//             Box::new(network),
//             Box::new(crates),
//         );

//         let cargo = vec![String::from("name = test")];

//         let result = add.add_dependency_to_crate(&cargo).unwrap();
//         assert_eq!(cargo.len() + 2, result.len());

//         assert_eq!(result[result.len() - 2], DEPENDENCIES_KEY);

//         let mut name_version = name.clone();
//         name_version.push_str(" = \"");
//         name_version.push_str(&version);
//         name_version.push_str("\"");
//         assert_eq!(result[result.len() - 1], name_version);
//     }

//     #[test]
//     fn handle_crate_write_error() {
//         let error_message = String::from("mock error");
//         let network = NetworkSimulator::new(vec![String::from("")], error_message.clone(), false);
//         let crates =
//             CratesSimulator::new(vec![String::from("")], error_message.clone(), false, true);

//         let mut add = Add::new(
//             String::from(""),
//             String::from(""),
//             Box::new(network),
//             Box::new(crates),
//         );
//         let result = add.crates_dependency();
//         assert!(result.is_err())
//     }
// }
