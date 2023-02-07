#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fmt, collections::HashMap};

use eframe::egui;
use egui_extras::RetainedImage;

fn load_images() -> HashMap<LogicGates, [RetainedImage; 3]> {
    let mut img_map: HashMap<LogicGates, [RetainedImage; 3]> = HashMap::new();
    img_map.insert(LogicGates::AND, [
        RetainedImage::from_image_bytes(
            "../assets/logo.png",
            include_bytes!("../assets/logo.png"),
        ).unwrap(),
        RetainedImage::from_image_bytes(
            "../assets/logo.png",
            include_bytes!("../assets/logo.png"),
        ).unwrap(),
        RetainedImage::from_image_bytes(
            "../assets/logo.png",
            include_bytes!("../assets/logo.png"),
        ).unwrap()
    ]);
    img_map
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn configure_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    // Possible to define styles in here
    ctx.set_style(style);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum LogicGates {
    AND,
    OR,
    XOR,
    NOT,
    NAND,
    NOR,
    XNOR
}

impl fmt::Display for LogicGates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogicGates::AND => {
                write!(f, "AND")
            }
            LogicGates::OR => {
                write!(f, "OR")
            }
            LogicGates::XOR => {
                write!(f, "XOR")
            },
            LogicGates::NOT => {
                write!(f, "NOT")
            },
            LogicGates::NAND => {
                write!(f, "NAND")
            },
            LogicGates::NOR => {
                write!(f, "NOR")
            },
            LogicGates::XNOR => {
                write!(f, "XNOR")
            },
        }
    }
}

impl LogicGates { 
    fn compute(&self, a:bool, b:bool) -> bool {
        match *self {
            LogicGates::AND => {
                a && b
            }
            LogicGates::OR => {
                a || b
            }
            LogicGates::XOR => {
                a ^ b
            },
            LogicGates::NOT => {
                !a
            },
            LogicGates::NAND => {
                !(a && b)
            },
            LogicGates::NOR => {
                !(a || b)
            },
            LogicGates::XNOR => {
                !(a ^ b)
            },
        }
    }
}

struct LogikgatterApp {
    selected_gate: LogicGates,
    gate_images: HashMap<LogicGates, [RetainedImage; 3]>,
    input_a: bool,
    input_b: bool,
    output: bool,
}

impl LogikgatterApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_styles(&cc.egui_ctx);
        let img_map: HashMap<LogicGates, [RetainedImage; 3]> = load_images();
        Self {
            selected_gate: LogicGates::AND,
            gate_images: img_map,
            input_a: false,
            input_b: false,
            output: false,
        }
    }
}

impl eframe::App for LogikgatterApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.output = LogicGates::compute(&self.selected_gate, self.input_a, self.input_b);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::from_main_dir_and_cross_align(egui::Direction::TopDown, egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("Logikgatter").heading().color(egui::Color32::from_rgb(255, 255, 255)));
                ui.label("Wähle ein Logikgatter aus und setze Input A / B auf High (Checked) oder Low (Unchecked).");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Logikgatter auswählen:");
                    egui::ComboBox::from_id_source(0)
                        .selected_text(self.selected_gate.to_string())
                        .show_ui(ui, |ui| {
                            for gate in [LogicGates::AND, LogicGates::OR, LogicGates::XOR, LogicGates::NOT, LogicGates::NAND, LogicGates::NOR, LogicGates::XNOR] {
                                ui.selectable_value(&mut self.selected_gate, gate, gate.to_string());
                            }
                        });
                        ui.checkbox(&mut self.input_a, "Input A");
                        if self.selected_gate != LogicGates::NOT {
                            ui.checkbox(&mut self.input_b, "Input B");
                        }
                });
                ui.horizontal(|ui| {
                    for image in self.gate_images.get(&self.selected_gate).unwrap() {
                        image.show_size(ui, egui::vec2(640.0/3.15, 640.0/3.15));
                    }
                });
                ui.label(format!("Output: {:}", self.output.to_string()));
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        resizable: false,
        icon_data: Some(load_icon("assets/logo_small.png")),
        ..Default::default()
    };
    
    eframe::run_native(
        "Logikgatter Erklärprogramm",
        options,
        Box::new(|cc| Box::new(LogikgatterApp::new(cc))),
    );
}
