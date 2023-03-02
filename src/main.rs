#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{collections::HashMap};

use eframe::epaint::{TextShape, Fonts};
use egui::{Color32, Pos2, text::LayoutJob, TextFormat, FontId, FontFamily, Rect, menu};
use egui_extras::{RetainedImage, TableBuilder, Column};
use strum_macros::Display;

static ALL_GATES: [LogicGates; 7] = [LogicGates::AND, LogicGates::OR, LogicGates::XOR, LogicGates::NOT, LogicGates::NAND, LogicGates::NOR, LogicGates::XNOR];

fn load_images() -> HashMap<LogicGates, RetainedImage> {
    let mut img_map: HashMap<LogicGates, RetainedImage> = HashMap::new();
    /*
    // Geht nicht, da include_bytes!() ein String literal benötigt (ein zu compile time bekannten String)
    for gate in ALL_GATES {
        img_map.insert(gate, RetainedImage::from_image_bytes(
            format!("../assets/gate_imgs/bool_table/{}.png", gate.to_string()),
            include_bytes!(format!("../assets/gate_imgs/bool_table/{}.png", gate.to_string())),
        ).unwrap());
    }*/
    img_map.insert(LogicGates::AND, RetainedImage::from_image_bytes(
        "../assets/gate_imgs/AND.png",
        include_bytes!("../assets/gate_imgs/AND.png"),
    ).unwrap());
    img_map.insert(LogicGates::OR, RetainedImage::from_image_bytes(
        "../assets/gate_imgs/OR.png",
        include_bytes!("../assets/gate_imgs/OR.png"),
    ).unwrap());
    img_map.insert(LogicGates::XOR, RetainedImage::from_image_bytes(
        "../assets/gate_imgs/XOR.png",
        include_bytes!("../assets/gate_imgs/XOR.png"),
    ).unwrap());
    img_map.insert(LogicGates::NOT, RetainedImage::from_image_bytes(
        "../assets/gate_imgs/NOT.png",
        include_bytes!("../assets/gate_imgs/NOT.png"),
    ).unwrap());
    img_map.insert(LogicGates::NAND, RetainedImage::from_image_bytes(
        "../assets/gate_imgs/NAND.png",
        include_bytes!("../assets/gate_imgs/NAND.png"),
    ).unwrap());
    img_map.insert(LogicGates::NOR, RetainedImage::from_image_bytes(
        "../assets/gate_imgs/NOR.png",
        include_bytes!("../assets/gate_imgs/NOR.png"),
    ).unwrap());
    img_map.insert(LogicGates::XNOR, RetainedImage::from_image_bytes(
        "../assets/gate_imgs/XNOR.png",
        include_bytes!("../assets/gate_imgs/XNOR.png"),
    ).unwrap());
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
    let style = (*ctx.style()).clone();
    // Possible to define styles in here
    ctx.set_style(style);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Display)]
enum LogicGates {
    AND,
    OR,
    XOR,
    NOT,
    NAND,
    NOR,
    XNOR
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
    gate_images: HashMap<LogicGates, RetainedImage>,
    input_a: bool,
    input_b: bool,
    output: bool,
    show_error_popup: bool,
}

impl LogikgatterApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_styles(&cc.egui_ctx);
        let img_map: HashMap<LogicGates, RetainedImage> = load_images();
        Self {
            selected_gate: LogicGates::AND,
            gate_images: img_map,
            input_a: false,
            input_b: false,
            output: false,
            show_error_popup: false,
        }
    }
}

fn gen_bool_label(val: bool, label: &str) -> egui::Label {
    if val {
        egui::Label::new(egui::RichText::new(format!("{}{}", label, "High")).color(Color32::GREEN))
    } else {
        egui::Label::new(egui::RichText::new(format!("{}{}", label, "Low")).color(Color32::RED))
    }
}

fn open_file(app: &mut LogikgatterApp, name: &str) {
    let file_open_result = open::that(name);
    match file_open_result {
        Ok(_) => {},
        Err(_) => app.show_error_popup = true,
    }
}

impl eframe::App for LogikgatterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.show_error_popup {
            egui::Window::new("Fehler - Datei nicht gefunden")
                .open(&mut self.show_error_popup)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.label("Die Datei konnte nicht geöffnet werden, da Sie sich nicht im Ordner des Programms befindet.");
                });
        }
        
        self.output = LogicGates::compute(&self.selected_gate, self.input_a, self.input_b);
        egui::CentralPanel::default().show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Hilfe", |ui| {
                    if ui.button("Theorie Logikgatter").clicked() {
                        open_file(self, "Theorie.pdf");
                    }
                    if ui.button("Bedienungsanleitung").clicked() {
                        open_file(self, "Bedienungsanleitung.pdf");
                    }
                })
            });
            ui.with_layout(egui::Layout::from_main_dir_and_cross_align(egui::Direction::TopDown, egui::Align::Center), |ui| {
                ui.label(egui::RichText::new("Logikgatter").heading().color(egui::Color32::from_rgb(255, 255, 255)));
                ui.label("Wähle ein Logikgatter aus und setze die Pegel A / B auf High (Checked) oder Low (Unchecked).");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Logikgatter auswählen:");
                    egui::ComboBox::from_id_source(0)
                        .selected_text(self.selected_gate.to_string())
                        .show_ui(ui, |ui| {
                            for gate in ALL_GATES {
                                ui.selectable_value(&mut self.selected_gate, gate, gate.to_string());
                            }
                        });
                        ui.checkbox(&mut self.input_a, "Eingang A");
                        if self.selected_gate != LogicGates::NOT {
                            ui.checkbox(&mut self.input_b, "Eingang B");
                        }
                });
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Wahrheitstabelle").color(egui::Color32::from_rgb(255, 255, 255)));
                        if self.selected_gate == LogicGates::NOT {
                            let bool_table = TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::initial(100.8))
                                .column(Column::initial(100.8));
    
                            bool_table
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.strong("A");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Ausgang");
                                    });
                                })
                                .body(|mut body| {
                                    for table_a in [true, false] {
                                        body.row(20.0, |mut row| {
                                            row.col(|ui| {
                                                ui.add(gen_bool_label(table_a, ""));
                                            });
                                            row.col(|ui| {
                                                ui.add(gen_bool_label(LogicGates::compute(&self.selected_gate, table_a, false), ""));
                                            });
                                        })
                                    }
                                });
                        } else {
                            let bool_table = TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                .column(Column::initial(64.5))
                                .column(Column::initial(64.5))
                                .column(Column::initial(64.5));
        
                            bool_table
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.strong("A");
                                    });
                                    header.col(|ui| {
                                        ui.strong("B");
                                    });
                                    header.col(|ui| {
                                        ui.strong("Ausgang");
                                    });
                                })
                                .body(|mut body| {
                                    for table_a in [true, false] {
                                        for table_b in [true, false] {
                                            body.row(20.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.add(gen_bool_label(table_a, ""));
                                                });
                                                row.col(|ui| {
                                                    ui.add(gen_bool_label(table_b, ""));
                                                });
                                                row.col(|ui| {
                                                    ui.add(gen_bool_label(LogicGates::compute(&self.selected_gate, table_a, table_b), ""));
                                                });
                                            })
                                        }
                                    }
                                });
                        }
                    });
                    self.gate_images.get(&self.selected_gate).unwrap().show_size(ui, egui::vec2(640.0/3.15, 640.0/3.15));
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Karnaugh-Veitch-Diagramm").color(egui::Color32::from_rgb(255, 255, 255)));
                        // Table
                        ui.push_id(100, |ui| {
                            if self.selected_gate == LogicGates::NOT {
                                let kv_diagram = TableBuilder::new(ui)
                                    .striped(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .column(Column::initial(90.8))
                                    .column(Column::initial(90.8));
                                kv_diagram
                                    .header(20.0, |mut header| {
                                        header.col(|_ui| {});
                                        header.col(|_ui| {});
                                    })
                                    .body(|mut body| {
                                        body.row(20.0, |mut row| {
                                            row.col(|ui| {
                                                ui.strong("1");
                                            });
                                            row.col(|ui| {
                                                ui.strong("0");
                                            });
                                        });
                                        body.row(20.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label("0");
                                            });
                                            row.col(|ui| {
                                                ui.label("1");
                                            });
                                        });
                                    });
                            } else {
                                let kv_diagram = TableBuilder::new(ui)
                                    .striped(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .column(Column::initial(57.9))
                                    .column(Column::initial(57.9))
                                    .column(Column::initial(57.9));
            
                                kv_diagram
                                    .header(20.0, |mut header| {
                                        header.col(|_ui| {});
                                        header.col(|_ui| {});
                                        header.col(|_ui| {});
                                    })
                                    .body(|mut body| {
                                        body.row(20.0, |mut row| {
                                            row.col(|_ui| {});
                                            row.col(|ui| {
                                                ui.strong("1");
                                            });
                                            row.col(|ui| {
                                                ui.strong("0");
                                            });

                                        });
                                        for val in [1, 0] {
                                            body.row(20.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.strong(val.to_string());
                                                });
                                                row.col(|ui| {
                                                    ui.label(if LogicGates::compute(&self.selected_gate, true, val == 1) {
                                                        "1"
                                                    } else {
                                                        "0"
                                                    });
                                                });
                                                row.col(|ui| {
                                                    ui.label(if LogicGates::compute(&self.selected_gate, false, val == 1) {
                                                        "1"
                                                    } else {
                                                        "0"
                                                    });
                                                });
    
                                            });
                                        }
                                    });
                            }
                        });
                    });
                });
                ui.add(gen_bool_label(self.output, "Ausgangspegel: "));
            });
            let painter = ui.painter();
            // Labels
            let mut job = LayoutJob::default();
            job.append(
                "A",
                0.0,
                TextFormat {
                    font_id: FontId::new(14.0, FontFamily::Proportional),
                    color: Color32::WHITE,
                    ..Default::default()
                },
            );
            let mut text_shape = TextShape::new(Pos2::new(529.0, 118.5), Fonts::layout_job(&ctx.fonts(), job));
            painter.add(text_shape);
            if self.selected_gate != LogicGates::NOT {
                job = LayoutJob::default();
                job.append(
                    "B",
                    0.0,
                    TextFormat {
                        font_id: FontId::new(14.0, FontFamily::Proportional),
                        color: Color32::WHITE,
                        ..Default::default()
                    },
                );
                text_shape = TextShape::new(Pos2::new(415.0, 178.5), Fonts::layout_job(&ctx.fonts(), job));
                text_shape.angle = -3.14/2.0;
                painter.add(text_shape);
            }
            // Mark rows and columns according to inputs
            if self.selected_gate != LogicGates::NOT {
                let col_width = 64.5;
                let row_height = 22.5;
                let input_a_x = 435.0 + if self.input_a {
                    col_width
                } else {
                    col_width * 2.0
                };
                let input_a_y = 162.5;
                painter.rect_stroke(Rect::from_two_pos(Pos2::new(input_a_x, input_a_y), Pos2::new(input_a_x + col_width, input_a_y + row_height * 2.0)), 1.0, (2.0, Color32::BLUE));
                
                let input_b_x = 435.0 + col_width;
                let input_b_y = 162.5 + if self.input_b {
                    0.0
                } else {
                    row_height
                };
                painter.rect_stroke(Rect::from_two_pos(Pos2::new(input_b_x, input_b_y), Pos2::new(input_b_x + col_width * 2.0, input_b_y + row_height)), 1.0, (2.0, Color32::LIGHT_GRAY));
            } else {
                let col_width = 97.8;
                let tmp_x = 435.0 + if self.input_a {
                    0.0
                } else {
                    col_width
                };
                let tmp_y = 162.5;
                painter.rect_stroke(Rect::from_two_pos(Pos2::new(tmp_x, tmp_y), Pos2::new(tmp_x + col_width, tmp_y + 20.0)), 1.0, (1.0, Color32::LIGHT_BLUE));
            }
            // Draw circles to indicate input and output in image
            if self.selected_gate != LogicGates::NOT {
                painter.circle_filled(Pos2::new(320.25, 144.5), 5.0, if self.input_a {
                    Color32::GREEN
                } else {
                    Color32::RED
                });
                painter.circle_filled(Pos2::new(343.0, 144.5), 5.0, if self.input_b {
                    Color32::GREEN
                } else {
                    Color32::RED
                });
            } else {
                painter.circle_filled(Pos2::new(331.6, 144.5), 5.0, if self.input_a {
                    Color32::GREEN
                } else {
                    Color32::RED
                });
            }
            painter.circle_filled(Pos2::new(331.6, 258.5), 5.0, if self.output {
                Color32::GREEN
            } else {
                Color32::RED
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 350.0)),
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
