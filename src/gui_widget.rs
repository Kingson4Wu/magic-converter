use iced::widget::{button, column, container, row, text, progress_bar};
use iced::{Application, Command, Element, Length, Theme, Alignment};
use std::path::PathBuf;
use rfd::FileDialog;
use std::sync::{mpsc, Arc};
use std::thread;

use crate::ConverterService;

#[derive(Debug, Clone)]
pub enum Message {
    SelectInputPath,
    SelectOutputPath,
    InputPathSelected(Option<PathBuf>),
    OutputPathSelected(Option<PathBuf>),
    ConvertSingleFile,
    ConvertDirectory,
    ConversionProgress(f32),
    ConversionComplete(Result<(), String>),
}

#[derive(Debug)]
pub struct ConverterGui {
    pub input_path: String,
    pub output_path: String,
    pub status_message: String,
    pub conversion_progress: f32,
    pub is_converting: bool,
    pub service: ConverterService,
}

impl Application for ConverterGui {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                input_path: String::new(),
                output_path: String::new(),
                status_message: String::from("Select a file or directory to convert"),
                conversion_progress: 0.0,
                is_converting: false,
                service: ConverterService::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Magic Converter")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectInputPath => {
                if let Some(path) = FileDialog::new()
                    .add_filter("MTS Video", &["MTS", "mts"])
                    .pick_file()
                {
                    self.input_path = path.to_string_lossy().into_owned();
                    self.status_message = String::from("Ready to convert");
                }
                Command::none()
            }
            Message::SelectOutputPath => {
                if let Some(path) = FileDialog::new()
                    .pick_folder()
                {
                    self.output_path = path.to_string_lossy().into_owned();
                }
                Command::none()
            }
            Message::InputPathSelected(Some(path)) => {
                self.input_path = path.to_string_lossy().into_owned();
                self.status_message = String::from("Ready to convert");
                Command::none()
            }
            Message::InputPathSelected(None) => {
                self.status_message = String::from("No file selected");
                Command::none()
            }
            Message::OutputPathSelected(Some(path)) => {
                self.output_path = path.to_string_lossy().into_owned();
                Command::none()
            }
            Message::OutputPathSelected(None) => Command::none(),
            Message::ConvertSingleFile => {
                if !self.is_converting {
                    self.is_converting = true;
                    self.conversion_progress = 0.0;
                    self.status_message = String::from("Converting...");
                    
                    let input = PathBuf::from(&self.input_path);
                    let output = if self.output_path.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(&self.output_path))
                    };

                    let service = self.service.clone();
                    let input_path = input.clone();

                    Command::perform(
                        async move {
                            let (tx, rx) = mpsc::channel();
                            let tx_progress = tx.clone();
                            let progress_callback = Arc::new(move |progress: f32| {
                                let _ = tx_progress.send(Message::ConversionProgress(progress));
                            });

                            // Spawn conversion thread
                            let tx_complete = tx.clone();
                            thread::spawn(move || {
                                let result = service.convert_file(&input_path, output.as_deref(), Some(progress_callback));
                                let _ = tx_complete.send(Message::ConversionComplete(result.map_err(|e| e.to_string())));
                            });

                            // Handle messages
                            let mut last_progress = Message::ConversionProgress(0.0);
                            while let Ok(message) = rx.recv() {
                                match message {
                                    Message::ConversionComplete(_) => return message,
                                    Message::ConversionProgress(_) => last_progress = message,
                                    _ => continue,
                                }
                            }
                            last_progress
                        },
                        std::convert::identity
                    )
                } else {
                    Command::none()
                }
            }
            Message::ConvertDirectory => {
                if !self.is_converting {
                    self.is_converting = true;
                    self.conversion_progress = 0.0;
                    self.status_message = String::from("Converting directory...");
                    
                    let input = PathBuf::from(&self.input_path);
                    let output = if self.output_path.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(&self.output_path))
                    };

                    let service = self.service.clone();
                    let input_path = input.clone();

                    Command::perform(
                        async move {
                            let (tx, rx) = mpsc::channel();
                            let tx_progress = tx.clone();
                            let progress_callback = Arc::new(move |progress: f32| {
                                let _ = tx_progress.send(Message::ConversionProgress(progress));
                            });

                            // Spawn conversion thread
                            let tx_complete = tx.clone();
                            thread::spawn(move || {
                                let result = service.convert_directory(&input_path, output.as_deref(), Some(progress_callback));
                                let _ = tx_complete.send(Message::ConversionComplete(result.map_err(|e| e.to_string())));
                            });

                            // Handle messages
                            let mut last_progress = Message::ConversionProgress(0.0);
                            while let Ok(message) = rx.recv() {
                                match message {
                                    Message::ConversionComplete(_) => return message,
                                    Message::ConversionProgress(_) => last_progress = message,
                                    _ => continue,
                                }
                            }
                            last_progress
                        },
                        std::convert::identity
                    )
                } else {
                    Command::none()
                }
            }
            Message::ConversionProgress(progress) => {
                self.conversion_progress = progress;
                self.status_message = format!("Converting... {}%", (progress * 100.0) as i32);
                Command::none()
            }
            Message::ConversionComplete(result) => {
                self.is_converting = false;
                match result {
                    Ok(_) => {
                        self.status_message = String::from("Conversion completed successfully");
                        self.conversion_progress = 1.0;
                    }
                    Err(e) => {
                        self.status_message = format!("Conversion failed: {}", e);
                        self.conversion_progress = 0.0;
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input_row = row![
            text("Input:").width(Length::Fixed(60.0)),
            text(&self.input_path).width(Length::Fill),
            button("Browse").on_press(Message::SelectInputPath),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let output_row = row![
            text("Output:").width(Length::Fixed(60.0)),
            text(&self.output_path).width(Length::Fill),
            button("Browse").on_press(Message::SelectOutputPath),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let button_row = row![
            button("Convert File")
                .on_press(Message::ConvertSingleFile)
                .width(Length::Fixed(100.0)),
            button("Convert Directory")
                .on_press(Message::ConvertDirectory)
                .width(Length::Fixed(120.0)),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let progress_bar = progress_bar(0.0..=1.0, self.conversion_progress)
            .width(Length::Fill);

        let content = column![
            input_row,
            output_row,
            button_row,
            progress_bar,
            text(&self.status_message),
        ]
        .spacing(20)
        .padding(20)
        .max_width(800);

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
        <Self as Application>::run(settings)
    }
}
