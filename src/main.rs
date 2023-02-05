#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::io::Write;
use std::path::PathBuf;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "MTextEditor",
        options,
        Box::new(|_cc| Box::new(MTextEditor::default())),
    );
}

struct MTextEditor {
    state: State,
}

impl eframe::App for MTextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&format!(
                "MTextEditor {}{}",
                if let State::Loaded(ref mut ct) = self.state {
                    if ct.buffer_text_box == ct.text_box {
                        ct.dirty = false;
                    } else {
                        ct.dirty = true;
                    }
                    if ct.dirty {
                        "- *"
                    } else {
                        "- "
                    }
                } else {
                    ""
                },
                if let State::Loaded(ct) = &self.state {
                    &ct.title.as_str()
                } else {
                    ""
                }
            ));
            ui.horizontal(|ui| {
                if ui.button("New").clicked() {
                    if let State::Loaded(ctn) = &self.state {
                        if !ctn.dirty {
                            self.state = State::Loaded(Content::default());
                        }
                    } else {
                        self.state = State::Loaded(Content::default());
                    }
                }
                if ui.button("Open").clicked() {
                    if let Some(fpath) = rfd::FileDialog::new().pick_file() {
                        self.state = State::Loaded(Content {
                            title: fpath
                                .file_name()
                                .expect("[Error] Invalid Destination")
                                .to_str()
                                .unwrap()
                                .to_string(),
                            text_box: std::fs::read_to_string(&fpath)
                                .expect("[Error] Reading File"),
                            path: Some(fpath),
                            ..Content::default()
                        });
                    }
                }

                if let State::Loaded(ref mut ctn) = self.state {
                    if ui.button("Save").clicked() {
                        let mut fpath = PathBuf::new();

                        let save = if let Some(fpath1) = ctn.path.as_ref() {
                            fpath = fpath1.clone();
                            true
                        } else {
                            if let Some(fpath2) = rfd::FileDialog::new().save_file() {
                                fpath = fpath2;
                                ctn.title = fpath
                                    .file_name()
                                    .expect("[Error] Invalid Destination")
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                true
                            } else {
                                false
                            }
                        };

                        if save {
                            let mut file =
                                std::fs::File::create(fpath).expect("[Error] Creating File");
                            file.write_all(ctn.text_box.as_bytes())
                                .expect("[Error] Writing File\nAborting...");
                            ctn.buffer_text_box = ctn.text_box.clone();
                            ctn.dirty = false;
                        }
                    }
                } else {
                    ui.add_enabled(false, egui::Button::new("Save"));
                }
            });

            if let State::Loaded(ref mut inp) = self.state {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut inp.text_box).code_editor(),
                    );
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("MTextEditor");
                });
            }
        });
    }
}

impl Default for MTextEditor {
    fn default() -> Self {
        MTextEditor {
            state: State::Loading,
        }
    }
}

enum State {
    Loading,
    Loaded(Content),
}

struct Content {
    title: String,
    text_box: String,
    buffer_text_box: String,
    path: Option<PathBuf>,
    dirty: bool,
}

impl Default for Content {
    fn default() -> Self {
        Content {
            title: String::from("New File"),
            text_box: String::new(),
            buffer_text_box: String::new(),
            path: None,
            dirty: true,
        }
    }
}
