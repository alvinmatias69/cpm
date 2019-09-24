mod add;
mod crates;
mod network;

fn main() {
    let crate_helper = crates::helper::Helper {};
    let crates = crates::crates::Crates::new(Box::new(crate_helper));

    let network_helper = network::helper::Helper {};
    let network = network::network::Network::new(Box::new(network_helper));

    // test case
    let mut adder = add::add::Add::new(
        String::from("tokio"),
        String::from("latest"),
        Box::new(network),
        Box::new(crates),
    );

    match adder.crates_dependency() {
        Err(e) => println!("Error: {}", e),
        Ok(()) => println!("Success!"),
    }
}
