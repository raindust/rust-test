use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(c) = matches.value_of("config") {
        println!("Value for -c {}", c);
    }
}
