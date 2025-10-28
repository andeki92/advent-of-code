// Visualization framework for Advent of Code
//
// This crate provides utilities for creating interactive visualizations
// of Advent of Code solutions using egui.
//
// Example usage:
// ```
// use aoc_viz::{VizApp, VizState};
//
// struct MyViz {
//     // Your visualization state
// }
//
// impl VizApp for MyViz {
//     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//         // Your visualization logic
//     }
// }
// ```

use eframe::egui;

/// Base trait for visualization applications
pub trait VizApp {
    /// Update the visualization
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);

    /// Optional: Called once before the first frame
    fn setup(&mut self, _ctx: &egui::Context) {}

    /// Optional: Called when the app is being closed
    fn on_exit(&mut self) {}
}

/// Run a visualization application
pub fn run_viz<T: VizApp + 'static>(
    title: &str,
    mut app: T,
) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title(title),
        ..Default::default()
    };

    eframe::run_simple_native(title, options, move |ctx, frame| {
        app.update(ctx, frame);
    })
}

/// Helper for grid-based visualizations
pub mod grid {
    use super::*;

    /// A simple 2D grid renderer
    pub struct GridRenderer {
        pub cell_size: f32,
        pub grid_width: usize,
        pub grid_height: usize,
    }

    impl GridRenderer {
        pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
            Self {
                cell_size,
                grid_width: width,
                grid_height: height,
            }
        }

        /// Draw a colored cell at (x, y)
        pub fn draw_cell(
            &self,
            painter: &egui::Painter,
            x: usize,
            y: usize,
            color: egui::Color32,
        ) {
            let rect = egui::Rect::from_min_size(
                egui::pos2(x as f32 * self.cell_size, y as f32 * self.cell_size),
                egui::vec2(self.cell_size, self.cell_size),
            );
            painter.rect_filled(rect, 0.0, color);
        }

        /// Draw grid lines
        pub fn draw_grid_lines(&self, painter: &egui::Painter, color: egui::Color32) {
            let stroke = egui::Stroke::new(1.0, color);

            // Vertical lines
            for x in 0..=self.grid_width {
                let x_pos = x as f32 * self.cell_size;
                painter.line_segment(
                    [
                        egui::pos2(x_pos, 0.0),
                        egui::pos2(x_pos, self.grid_height as f32 * self.cell_size),
                    ],
                    stroke,
                );
            }

            // Horizontal lines
            for y in 0..=self.grid_height {
                let y_pos = y as f32 * self.cell_size;
                painter.line_segment(
                    [
                        egui::pos2(0.0, y_pos),
                        egui::pos2(self.grid_width as f32 * self.cell_size, y_pos),
                    ],
                    stroke,
                );
            }
        }
    }
}

/// Helper for animation control
pub mod animation {
    /// Animation controller with play/pause and speed control
    pub struct AnimationController {
        pub playing: bool,
        pub speed: f32,
        pub current_frame: usize,
        pub total_frames: usize,
    }

    impl AnimationController {
        pub fn new(total_frames: usize) -> Self {
            Self {
                playing: false,
                speed: 1.0,
                current_frame: 0,
                total_frames,
            }
        }

        /// Update the current frame
        pub fn update(&mut self, delta_time: f32) {
            if self.playing && self.total_frames > 0 {
                let frame_increment = (delta_time * 60.0 * self.speed) as usize;
                self.current_frame = (self.current_frame + frame_increment) % self.total_frames;
            }
        }

        /// Reset to first frame
        pub fn reset(&mut self) {
            self.current_frame = 0;
        }

        /// Draw UI controls
        pub fn ui(&mut self, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                if ui.button(if self.playing { "⏸" } else { "▶" }).clicked() {
                    self.playing = !self.playing;
                }

                if ui.button("⏹").clicked() {
                    self.reset();
                    self.playing = false;
                }

                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed, 0.1..=10.0).logarithmic(true));

                ui.label(format!("Frame: {}/{}", self.current_frame, self.total_frames));
            });
        }
    }
}

#[cfg(feature = "export")]
pub mod export {
    /// GIF export functionality
    /// TODO: Implement GIF export using image and gif crates
    pub fn export_gif(_frames: &[Vec<u8>], _path: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!("GIF export not yet implemented")
    }
}
