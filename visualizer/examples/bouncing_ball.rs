//! A simple demo showing a bouncing ball in a grid
//!
//! Run with: cargo run --example bouncing_ball -p aoc-visualizer
//!
//! Controls:
//! - Space: Play/Pause
//! - R: Reset

use aoc_visualizer::{VizApp, VizConfig, animation::AnimationController, grid::GridRenderer, ui};
use eframe::egui;

struct BouncingBallDemo {
    grid: GridRenderer,
    controller: AnimationController,
    ball_x: f32,
    ball_y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl BouncingBallDemo {
    fn new() -> Self {
        let grid_size = 20;
        Self {
            grid: GridRenderer::new(grid_size, grid_size, 32.0),
            controller: AnimationController::new(0),
            ball_x: 10.0,
            ball_y: 10.0,
            velocity_x: 0.3,
            velocity_y: 0.2,
        }
    }

    fn update_physics(&mut self) {
        if self.controller.playing {
            self.ball_x += self.velocity_x * self.controller.speed;
            self.ball_y += self.velocity_y * self.controller.speed;

            let max_x = (self.grid.grid_width - 1) as f32;
            let max_y = (self.grid.grid_height - 1) as f32;

            if self.ball_x < 0.0 {
                self.ball_x = 0.0;
                self.velocity_x = self.velocity_x.abs();
            } else if self.ball_x > max_x {
                self.ball_x = max_x;
                self.velocity_x = -self.velocity_x.abs();
            }

            if self.ball_y < 0.0 {
                self.ball_y = 0.0;
                self.velocity_y = self.velocity_y.abs();
            } else if self.ball_y > max_y {
                self.ball_y = max_y;
                self.velocity_y = -self.velocity_y.abs();
            }
        }
    }
}

impl VizApp for BouncingBallDemo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.controller.handle_keyboard(ctx);
        self.controller.update(ctx.input(|i| i.stable_dt));
        self.update_physics();

        egui::CentralPanel::default()
            .frame(egui::Frame::none()) // Remove default frame/padding
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // Standard control bar
                    ui::control_bar(ui, self.controller.playing, self.controller.speed);

                    // Begin frame - this handles background, offset, etc.
                    let (_response, painter) = self.grid.begin_frame(
                        ui,
                        egui::Color32::from_rgb(15, 15, 35), // Dark blue background
                    );

                    // Draw grid lines
                    self.grid
                        .draw_grid_lines(&painter, egui::Color32::from_rgb(40, 40, 60));

                    let ball_x = self.ball_x as usize;
                    let ball_y = self.ball_y as usize;

                    // Draw trail
                    if self.controller.playing {
                        let trail_x = (self.ball_x - self.velocity_x) as i32;
                        let trail_y = (self.ball_y - self.velocity_y) as i32;
                        if trail_x >= 0
                            && trail_x < self.grid.grid_width as i32
                            && trail_y >= 0
                            && trail_y < self.grid.grid_height as i32
                        {
                            self.grid.draw_cell(
                                &painter,
                                trail_x as usize,
                                trail_y as usize,
                                egui::Color32::from_rgba_unmultiplied(255, 215, 0, 80),
                            );
                        }
                    }

                    // Draw glow
                    self.grid.draw_cell(
                        &painter,
                        ball_x,
                        ball_y,
                        egui::Color32::from_rgba_unmultiplied(255, 215, 0, 60),
                    );

                    // Draw ball (smaller with rounded corners)
                    self.grid.draw_cell_padded(
                        &painter,
                        ball_x,
                        ball_y,
                        2.0, // padding
                        egui::Color32::from_rgb(255, 215, 0),
                        4.0, // rounding
                    );
                });
            });

        ctx.request_repaint();
    }

    fn setup(&mut self, _ctx: &egui::Context) {
        println!("🎄 Bouncing Ball Demo");
        println!("Space: Play/Pause | R: Reset");
    }
}

fn main() -> Result<(), eframe::Error> {
    let grid_size = 20.0 * 32.0; // 20 cells × 32 pixels
    let config = VizConfig::new("Bouncing Ball Demo").with_size(grid_size, grid_size + 40.0); // Extra space for status text

    aoc_visualizer::run_viz_with_config(config, BouncingBallDemo::new())
}
