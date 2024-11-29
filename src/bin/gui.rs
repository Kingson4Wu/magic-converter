use iced::{Sandbox, Settings};
use magic_converter::gui::ConverterGui;

fn main() -> iced::Result {
    ConverterGui::run(Settings::default())
}
