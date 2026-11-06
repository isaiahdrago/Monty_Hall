use eframe::egui;
use egui::Color32;
use std::time::{Duration, Instant};

#[path = "Game/puzzle.rs"]
mod puzzle;
#[path = "Game/runner.rs"]
mod runner;

pub struct DoorPuzzleApp {
    puzzle: puzzle::Puzzle,
    runner: Option<runner::Runner>,
    game_type: Option<String>,
    reveal_at: Option<Instant>,
    revealed_door: Option<usize>,
    game_over: bool,
    iterations_input: usize,
    switch_times_input: usize,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn start() -> Result<(), wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast;

    let window =
        web_sys::window().ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser window"))?;
    let document = window
        .document()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("No browser document"))?;
    let canvas = document
        .get_element_by_id("the_canvas_id")
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("Canvas not found"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| wasm_bindgen::JsValue::from_str("Element is not a canvas"))?;

    eframe::WebRunner::new()
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|_cc| Ok(Box::new(DoorPuzzleApp::default()))),
        )
        .await
}

impl Default for DoorPuzzleApp {
    fn default() -> Self {
        Self {
            puzzle: puzzle::Puzzle::new(),
            runner: None,
            game_type: None,
            reveal_at: None,
            revealed_door: None,
            game_over: false,
            iterations_input: 1000,
            switch_times_input: 1,
        }
    }
}

impl eframe::App for DoorPuzzleApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(ui.style()).fill(egui::Color32::from_rgb(18, 35, 72)))
            .show(ui, |ui| {
                if let Some(deadline) = self.reveal_at {
                    let now = Instant::now();
                    if now >= deadline {
                        self.revealed_door = self.puzzle.reveal();
                        self.reveal_at = None;
                    } else {
                        ui.ctx().request_repaint_after(deadline - now);
                    }
                }

                ui.visuals_mut().override_text_color = Some(Color32::BLACK);
                ui.heading(egui::RichText::new("Door Puzzle").color(Color32::WHITE));
                ui.add_space(12.0);

                if self.game_type.is_none() {
                    ui.label(egui::RichText::new("Choose a game type:").color(Color32::WHITE));
                    if (ui.button(egui::RichText::new("Single Game").color(Color32::BLACK))).clicked() {
                        self.game_type = Some("single".to_string());
                    }
                    if (ui.button(egui::RichText::new("Run Statistics").color(Color32::BLACK))).clicked() {
                        self.game_type = Some("statistics".to_string());
                    }
                } else if self.game_type.as_deref() == Some("statistics") {
                    if let Some(runner) = &mut self.runner {
                            ui.label(
                            egui::RichText::new(format!(
                                    "Total Games: {}, Switch Games: {}, Stay Games: {}",
                                    runner.iterations, runner.switch_games, runner.stay_games
                                ))
                                .color(Color32::WHITE),
                            );
                            ui.label(
                            egui::RichText::new(format!(
                                    "Total Wins: {}, Switch Wins: {}, Stay Wins: {}",
                                    runner.wins, runner.switch_wins, runner.stay_wins
                                ))
                                .color(Color32::WHITE),
                            );
                            ui.label(
                            egui::RichText::new(format!(
                                    "Percentage of Wins: {:.2}%, Percentage of Switch Wins: {:.2}%, Percentage of Stay Wins: {:.2}%",
                                    (runner.wins as f64 / runner.iterations as f64) * 100.0,
                                    (runner.switch_wins as f64 / runner.switch_games as f64) * 100.0,
                                    (runner.stay_wins as f64 / runner.stay_games as f64) * 100.0
                                ))
                                .color(Color32::WHITE),
                            );
                            if (ui.button(egui::RichText::new("Return").color(Color32::BLACK))).clicked() {
                                self.runner = None;
                                self.game_type = None;
                            }
                            if (ui.button(egui::RichText::new("Play Again").color(Color32::BLACK))).clicked() {
                                self.runner = None;
                            }
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Enter the Number of Iterations:").color(Color32::WHITE));
                            ui.add(egui::DragValue::new(&mut self.iterations_input).range(1..=100000));
                        });
                        ui.label(egui::RichText::new("Enter the How Often it Switches:").color(Color32::WHITE));
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("(0 means never, 1 means every time, 2 means every other time, etc.)").color(Color32::WHITE));
                            ui.add(egui::DragValue::new(&mut self.switch_times_input).range(0..=100000));
                        });
                        if (ui.button(egui::RichText::new("Run Statistics").color(Color32::BLACK))).clicked() {
                            let mut runner = runner::Runner::new(
                                self.iterations_input,
                                self.switch_times_input,
                            );
                            runner.run();
                            self.runner = Some(runner);
                        }
                    }
                } else if self.game_type.as_deref() == Some("single") {
                    if self.puzzle.choice.is_none() {
                        ui.label(egui::RichText::new("Choose a door:").color(Color32::WHITE));
                        if (ui.button(egui::RichText::new("Door 1").color(Color32::BLACK))).clicked() {
                            self.puzzle.set_initial_choice(0);
                            self.reveal_at = Some(Instant::now() + Duration::from_secs(1));
                            ui.ctx().request_repaint_after(Duration::from_secs(1));
                        }
                        if (ui.button(egui::RichText::new("Door 2").color(Color32::BLACK))).clicked() {
                            self.puzzle.set_initial_choice(1);
                            self.reveal_at = Some(Instant::now() + Duration::from_secs(1));
                            ui.ctx().request_repaint_after(Duration::from_secs(1));
                        }
                        if (ui.button(egui::RichText::new("Door 3").color(Color32::BLACK))).clicked() {
                            self.puzzle.set_initial_choice(2);
                            self.reveal_at = Some(Instant::now() + Duration::from_secs(1));
                            ui.ctx().request_repaint_after(Duration::from_secs(1));
                        }
                    } else if self.reveal_at.is_some() {
                        ui.label(egui::RichText::new("Revealing a door...").color(Color32::WHITE));
                    } else if !self.game_over {
                        if let Some(door) = self.revealed_door {
                            ui.label(
                                egui::RichText::new(format!("Door {} is empty.", door + 1))
                                    .color(Color32::WHITE),
                            );
                        }
                        ui.label(
                            egui::RichText::new("Would you like to switch your choice?")
                                .color(Color32::WHITE),
                        );
                        if (ui.button(egui::RichText::new("Switch").color(Color32::BLACK))).clicked() {
                            self.puzzle.switch_choice();
                            self.game_over = true;
                        }
                        if (ui.button(egui::RichText::new("Stay").color(Color32::BLACK))).clicked() {
                            self.game_over = true;
                        }
                    } else {
                        let win = self.puzzle.end_game();
                        if win {
                            ui.label(egui::RichText::new("You won!").color(Color32::WHITE));
                        } else {
                            ui.label(egui::RichText::new("You lost.").color(Color32::WHITE));
                        }
                        if (ui.button(egui::RichText::new("Return").color(Color32::BLACK))).clicked() {
                            self.puzzle.reset();
                            self.reveal_at = None;
                            self.revealed_door = None;
                            self.game_over = false;
                            self.game_type = None;
                        }
                        if (ui.button(egui::RichText::new("Play Again").color(Color32::BLACK))).clicked() {
                            self.puzzle.reset();
                            self.reveal_at = None;
                            self.revealed_door = None;
                            self.game_over = false;
                        }
                    }
                }
            });
    }
}
