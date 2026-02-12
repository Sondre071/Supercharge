use eframe::egui;
use std::process;

mod blobstorage;
mod openrouter;
mod scripts;
mod shared;

use openrouter::{OpenRouterAction, OpenRouterUi};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Supercharge",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    screen: Screen,
}

enum Screen {
    MainMenu,
    OpenRouter(OpenRouterUi),
    BlobStorage,
    Scripts,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            screen: Screen::MainMenu,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.screen {
            Screen::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Supercharge");
                    ui.add_space(20.0);

                    if ui.button("OpenRouter").clicked() {
                        self.screen = Screen::OpenRouter(OpenRouterUi::new());
                    }

                    if ui.button("BlobStorage").clicked() {
                        self.screen = Screen::BlobStorage;
                    }

                    if ui.button("Scripts").clicked() {
                        self.screen = Screen::Scripts;
                    }

                    if ui.button("Exit").clicked() {
                        process::exit(0);
                    }
                });
            }

            Screen::OpenRouter(ui) => {
                let action = ui.draw(ctx);

                if let OpenRouterAction::GoBack = action {
                    self.screen = Screen::MainMenu;
                }
            }

            Screen::BlobStorage => {
                self.screen = Screen::MainMenu;
            }

            Screen::Scripts => {
                self.screen = Screen::MainMenu;
            }
        };
    }
}
