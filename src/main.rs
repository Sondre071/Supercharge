use std::process;
use eframe::egui;

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
    current_module: Screen,
}

enum Screen {
    MainMenu,
    OpenRouter(OpenRouterUi),
    BlobStorage,
    Scripts,
}

enum GoTo {
    None,
    MainMenu,
    OpenRouter,
    BlobStorage,
    Scripts,
    Exit,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            current_module: Screen::MainMenu,
        }
    }
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn handle_go_to(&mut self, to: GoTo) {
        match to {
            GoTo::None => {}

            GoTo::MainMenu => {
                self.current_module = Screen::MainMenu;
            }

            GoTo::OpenRouter => {
                self.current_module = Screen::OpenRouter(OpenRouterUi::new());
            }

            GoTo::BlobStorage => {
                self.current_module = Screen::BlobStorage;
            }

            GoTo::Scripts => {
                self.current_module = Screen::Scripts;
            }

            GoTo::Exit => {
                process::exit(0);
            }
        }
    }

    fn ui_main_menu(&mut self, ctx: &egui::Context) -> GoTo {
        let mut action = GoTo::None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Supercharge");
            ui.add_space(20.0);

            if ui.button("OpenRouter").clicked() {
                action = GoTo::OpenRouter;
            }

            if ui.button("BlobStorage").clicked() {
                action = GoTo::BlobStorage;
            }

            if ui.button("Scripts").clicked() {
                action = GoTo::Scripts;
            }

            if ui.button("Exit").clicked() {
                action = GoTo::Exit;
            }
        });

        action
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let action = match &mut self.current_module {
            Screen::MainMenu => self.ui_main_menu(ctx),

            Screen::OpenRouter(ui) => {
                let openrouter_action = ui.ui(ctx);

                match openrouter_action {
                    OpenRouterAction::None => GoTo::None,
                    OpenRouterAction::GoBack => GoTo::MainMenu,
                }
            }

            Screen::BlobStorage => GoTo::None,
            Screen::Scripts => GoTo::None,
        };

        self.handle_go_to(action);
    }
}
