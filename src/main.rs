use iced::{Settings, Application};
use clap::{App, load_yaml};

// extern crate vpsman;
use vpsman::todos::*;

fn main() {
    let yaml = load_yaml!("config.yml");
    let m = App::from(yaml).get_matches();

    if let Some(mode) = m.value_of("mode") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _ => unreachable!(),
        }
    } else {
        println!("--mode <MODE> wasn't used...");
    }

    match m.subcommand_name() {
        Some("gui") => {
            gui().expect("");
            println!("'myapp add' was used")
        },
        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}

pub fn gui() -> iced::Result {
    <todos::Todos as Application>::run(Settings::default())
}
