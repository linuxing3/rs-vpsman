use iced::{Settings, Application};

use rs-vpsman::gui::Counter;

pub fn main() -> iced::Result {
    <Counter as Application>::run(Settings::default())
}

