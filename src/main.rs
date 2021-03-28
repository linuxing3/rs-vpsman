use iced::{Settings, Application};
use rs-vpsman::gui::Counter;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("config.yml");
    let opts = App::from(yaml).get_matches();

    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    match opts.gui {
        _ => gui(),
    }
}

pub fn gui() -> iced::Result {
    <Counter as Application>::run(Settings::default())
}

