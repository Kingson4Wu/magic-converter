use iced::widget::{self, button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length, Sandbox};
use std::path::PathBuf;

use crate::ConverterService;

#[derive(Debug, Clone)]
pub enum Message {
    InputPathChanged(String),
    OutputPathChanged(String),
    ConvertSingleFile,
    ConvertDirectory,
    ConversionComplete(Result<(), String>),
}

#[derive(Debug)]
pub struct ConverterGui {
    pub input_path: String,
    pub output_path: String,
    pub status_message: String,
    pub service: ConverterService,
}

impl Sandbox for ConverterGui {
    type Message = Message;

    fn new() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            status_message: String::from("Ready to convert"),
            service: ConverterService::new(),
        }
    }

    fn title(&self) -> String {
        String::from("MTS to MP4 Converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputPathChanged(path) => {
                self.input_path = path;
            }
            Message::OutputPathChanged(path) => {
                self.output_path = path;
            }
            Message::ConvertSingleFile => {
                let input = PathBuf::from(&self.input_path);
                let output = if self.output_path.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(&self.output_path))
                };

                match self.service.convert_file(&input, output.as_deref()) {
                    Ok(_) => {
                        self.status_message = format!("Successfully converted {}", input.display());
                    }
                    Err(e) => {
                        self.status_message = format!("Error: {}", e);
                    }
                }
            }
            Message::ConvertDirectory => {
                let input = PathBuf::from(&self.input_path);
                let output = if self.output_path.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(&self.output_path))
                };

                match self.service.convert_directory(&input, output.as_deref()) {
                    Ok(_) => {
                        self.status_message = format!("Successfully converted directory {}", input.display());
                    }
                    Err(e) => {
                        self.status_message = format!("Error: {}", e);
                    }
                }
            }
            Message::ConversionComplete(result) => {
                match result {
                    Ok(_) => self.status_message = String::from("Conversion completed successfully"),
                    Err(e) => self.status_message = format!("Conversion failed: {}", e),
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input_row = row![
            text("Input Path:").width(Length::Fixed(100.0)),
            text_input("Enter input path", &self.input_path)
                .on_input(Message::InputPathChanged)
                .padding(10)
        ]
        .padding(10)
        .align_items(Alignment::Center);

        let output_row = row![
            text("Output Path:").width(Length::Fixed(100.0)),
            text_input("Enter output path (optional)", &self.output_path)
                .on_input(Message::OutputPathChanged)
                .padding(10)
        ]
        .padding(10)
        .align_items(Alignment::Center);

        let button_row = row![
            button("Convert Single File")
                .on_press(Message::ConvertSingleFile)
                .padding(10),
            button("Convert Directory")
                .on_press(Message::ConvertDirectory)
                .padding(10)
        ]
        .spacing(20)
        .padding(10);

        let status = text(&self.status_message).size(16);

        let content = column![input_row, output_row, button_row, status]
            .spacing(10)
            .padding(20)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl ConverterGui {
    pub fn run(settings: iced::Settings<()>) -> iced::Result {
        <Self as Sandbox>::run(settings)
    }
}
