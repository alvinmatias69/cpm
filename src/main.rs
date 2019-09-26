mod add;
mod cli;
mod crates;
mod display;
mod network;

fn main() {
    if let Ok(result) = cli::get() {
        let crate_helper = crates::helper::Helper {};
        let crates = crates::crates::Crates::new(Box::new(crate_helper));

        let network_helper = network::helper::Helper {};
        let network = network::network::Network::new(Box::new(network_helper));

        let display = display::Display::new();

        match result {
            cli::Type::Add(crates_ver) => {
                let mut adder = add::add::Add::new(
                    crates_ver[0].clone(),
                    crates_ver[1].clone(),
                    Box::new(network),
                    Box::new(crates),
                    Box::new(display),
                );

                match adder.crates_dependency() {
                    Err(e) => println!("Error: {}", e),
                    Ok(()) => println!("Success!"),
                }
            }
        }
    }
}
