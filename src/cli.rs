use clap::{App, Arg};

pub enum Type {
    Add(Vec<String>),
}

pub fn get() -> Result<Type, String> {
    let matches = App::new("Cargo Package Manager")
        .version("1.0.0")
        .author("Matias Alvin <alvinmatias@protonmail.com>")
        .about("Small crates to manages your crates dependencies")
        .arg(
            Arg::with_name("TYPES")
                .help("Operation types")
                .required(true)
                .possible_values(&["add"]),
        )
        .arg(Arg::with_name("CRATES").help("Crates name").required(true))
        .get_matches();

    let crates = String::from(matches.value_of("CRATES").unwrap());

    match matches.value_of("TYPES").unwrap() {
        "add" => return Ok(Type::Add(parse_crates(crates))),
        _ => return Err(String::from("undefined types")),
    }
}

fn parse_crates(crates: String) -> Vec<String> {
    let split: Vec<&str> = crates.split("@").collect();

    let mut result: Vec<String> = Vec::new();
    result.push(String::from(split[0]));

    if split.len() > 1 {
        result.push(String::from(split[1]));
    } else {
        result.push(String::from("latest"));
    }

    result
}
