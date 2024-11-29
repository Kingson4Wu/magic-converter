use iced::widget::{button, column, container, row, text, progress_bar};
use iced::{Alignment, Element, Length, Sandbox};
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

impl Sandbox for ConverterGui {
    type Message = Message;

    fn new() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            status_message: String::from("Ready to convert"),
            conversion_progress: 0.0,
            is_converting: false,
            service: ConverterService::new(),
        }
    }

    fn title(&self) -> String {
        String::from("MTS to MP4 Converter")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SelectInputPath => {
                let file_dialog = FileDialog::new()
                    .set_title("Select Input File or Directory")
                    .pick_file();
                
                if let Some(path) = file_dialog {
                    self.input_path = path.to_string_lossy().into_owned();
                }
            }
            Message::SelectOutputPath => {
                let file_dialog = FileDialog::new()
                    .set_title("Select Output Directory")
                    .pick_folder();
                
                if let Some(path) = file_dialog {
                    self.output_path = path.to_string_lossy().into_owned();
                }
            }
            Message::InputPathSelected(Some(path)) => {
                self.input_path = path.to_string_lossy().into_owned();
            }
            Message::OutputPathSelected(Some(path)) => {
                self.output_path = path.to_string_lossy().into_owned();
            }
            Message::InputPathSelected(None) | Message::OutputPathSelected(None) => {}
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

                    // Create a channel for progress updates
                    let (progress_tx, progress_rx) = mpsc::channel();
                    let (complete_tx, complete_rx) = mpsc::channel();

                    // Create a progress callback
                    let progress_callback = Arc::new(move |progress: f32| {
                        let _ = progress_tx.send(progress);
                    });

                    // Spawn a thread for conversion
                    let service = self.service.clone();
                    thread::spawn(move || {
                        let result = service.convert_file(&input, output.as_deref(), Some(progress_callback));
                        let _ = complete_tx.send(result.map_err(|e| e.to_string()));
                    });

                    // Handle progress updates
                    thread::spawn(move || {
                        while let Ok(progress) = progress_rx.recv() {
                            // Update UI with progress
                            let _ = iced::futures::executor::block_on(async {
                                iced::Command::perform(
                                    async move { Message::ConversionProgress(progress) },
                                    std::convert::identity,
                                )
                            });
                        }

                        // Handle completion
                        if let Ok(result) = complete_rx.recv() {
                            let _ = iced::futures::executor::block_on(async {
                                iced::Command::perform(
                                    async move { Message::ConversionComplete(result) },
                                    std::convert::identity,
                                )
                            });
                        }
                    });
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

                    // Create a channel for progress updates
                    let (progress_tx, progress_rx) = mpsc::channel();
                    let (complete_tx, complete_rx) = mpsc::channel();

                    // Create a progress callback
                    let progress_callback = Arc::new(move |progress: f32| {
                        let _ = progress_tx.send(progress);
                    });

                    // Spawn a thread for conversion
                    let service = self.service.clone();
                    thread::spawn(move || {
                        let result = service.convert_directory(&input, output.as_deref(), Some(progress_callback));
                        let _ = complete_tx.send(result.map_err(|e| e.to_string()));
                    });

                    // Handle progress updates
                    thread::spawn(move || {
                        while let Ok(progress) = progress_rx.recv() {
                            // Update UI with progress
                            let _ = iced::futures::executor::block_on(async {
                                iced::Command::perform(
                                    async move { Message::ConversionProgress(progress) },
                                    std::convert::identity,
                                )
                            });
                        }

                        // Handle completion
                        if let Ok(result) = complete_rx.recv() {
                            let _ = iced::futures::executor::block_on(async {
                                iced::Command::perform(
                                    async move { Message::ConversionComplete(result) },
                                    std::convert::identity,
                                )
                            });
                        }
                    });
                }
            }
            Message::ConversionProgress(progress) => {
                self.conversion_progress = progress;
                self.status_message = format!("Converting... {}%", (progress * 100.0) as i32);
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
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input_row = row![
            text("Input Path:").width(Length::Fixed(100.0)),
            text(&self.input_path).width(Length::Fill),
            button("Browse...").on_press(Message::SelectInputPath)
        ]
        .padding(10)
        .align_items(Alignment::Center);

        let output_row = row![
            text("Output Path:").width(Length::Fixed(100.0)),
            text(&self.output_path).width(Length::Fill),
            button("Browse...").on_press(Message::SelectOutputPath)
        ]
        .padding(10)
        .align_items(Alignment::Center);

        let button_row = row![
            button("Convert Single File")
                .on_press(Message::ConvertSingleFile)
                .padding(10)
                .width(Length::Fixed(150.0))
                .style(if self.is_converting {
                    iced::theme::Button::Secondary
                } else {
                    iced::theme::Button::Primary
                }),
            button("Convert Directory")
                .on_press(Message::ConvertDirectory)
                .padding(10)
                .width(Length::Fixed(150.0))
                .style(if self.is_converting {
                    iced::theme::Button::Secondary
                } else {
                    iced::theme::Button::Primary
                })
        ]
        .spacing(20)
        .padding(10);

        let progress_bar = progress_bar(0.0..=1.0, self.conversion_progress)
            .width(Length::Fill);

        let status = text(&self.status_message).size(16);

        let content = column![
            input_row,
            output_row,
            button_row,
            progress_bar,
            status
        ]
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
