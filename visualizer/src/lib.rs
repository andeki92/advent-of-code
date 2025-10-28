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

/// Configuration for visualization window
pub struct VizConfig {
    pub width: f32,
    pub height: f32,
    pub title: String,
}

impl VizConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            width: 1280.0,
            height: 720.0,
            title: title.into(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

/// Run a visualization application
pub fn run_viz<T: VizApp + 'static>(title: &str, app: T) -> Result<(), eframe::Error> {
    let config = VizConfig::new(title);
    run_viz_with_config(config, app)
}

/// Run a visualization application with custom configuration
pub fn run_viz_with_config<T: VizApp + 'static>(
    config: VizConfig,
    mut app: T,
) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.width, config.height])
            .with_title(&config.title),
        ..Default::default()
    };

    eframe::run_simple_native(&config.title, options, move |ctx, frame| {
        app.update(ctx, frame);
    })
}

/// Helper for grid-based visualizations
pub mod grid {
    use super::*;

    /// A simple 2D grid renderer with automatic offset handling
    pub struct GridRenderer {
        pub cell_size: f32,
        pub grid_width: usize,
        pub grid_height: usize,
        offset: egui::Pos2,
    }

    impl GridRenderer {
        pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
            Self {
                cell_size,
                grid_width: width,
                grid_height: height,
                offset: egui::Pos2::ZERO,
            }
        }

        /// Set the offset for all drawing operations (called automatically by begin_frame)
        pub fn set_offset(&mut self, offset: egui::Pos2) {
            self.offset = offset;
        }

        /// Begin a new frame - returns painter ready for drawing
        /// Call this at the start of your rendering, then use draw_* methods
        pub fn begin_frame<'a>(
            &mut self,
            ui: &mut egui::Ui,
            bg_color: egui::Color32,
        ) -> (egui::Response, egui::Painter) {
            let (response, painter) = ui.allocate_painter(
                egui::vec2(
                    self.grid_width as f32 * self.cell_size,
                    self.grid_height as f32 * self.cell_size,
                ),
                egui::Sense::hover(),
            );

            self.offset = response.rect.min;
            let painter = painter.with_clip_rect(response.rect);

            // Draw background
            painter.rect_filled(response.rect, 0.0, bg_color);

            (response, painter)
        }

        /// Draw a colored cell at (x, y) with automatic offset
        pub fn draw_cell(&self, painter: &egui::Painter, x: usize, y: usize, color: egui::Color32) {
            let rect = egui::Rect::from_min_size(
                egui::pos2(
                    self.offset.x + x as f32 * self.cell_size,
                    self.offset.y + y as f32 * self.cell_size,
                ),
                egui::vec2(self.cell_size, self.cell_size),
            );
            painter.rect_filled(rect, 0.0, color);
        }

        /// Draw a colored cell with rounded corners
        pub fn draw_cell_rounded(
            &self,
            painter: &egui::Painter,
            x: usize,
            y: usize,
            color: egui::Color32,
            rounding: f32,
        ) {
            let rect = egui::Rect::from_min_size(
                egui::pos2(
                    self.offset.x + x as f32 * self.cell_size,
                    self.offset.y + y as f32 * self.cell_size,
                ),
                egui::vec2(self.cell_size, self.cell_size),
            );
            painter.rect_filled(rect, rounding, color);
        }

        /// Draw a cell with padding (useful for balls, rounded items)
        pub fn draw_cell_padded(
            &self,
            painter: &egui::Painter,
            x: usize,
            y: usize,
            padding: f32,
            color: egui::Color32,
            rounding: f32,
        ) {
            let rect = egui::Rect::from_min_size(
                egui::pos2(
                    self.offset.x + x as f32 * self.cell_size + padding,
                    self.offset.y + y as f32 * self.cell_size + padding,
                ),
                egui::vec2(
                    self.cell_size - padding * 2.0,
                    self.cell_size - padding * 2.0,
                ),
            );
            painter.rect_filled(rect, rounding, color);
        }

        /// Draw grid lines with automatic offset
        pub fn draw_grid_lines(&self, painter: &egui::Painter, color: egui::Color32) {
            let stroke = egui::Stroke::new(1.0, color);

            // Vertical lines
            for x in 0..=self.grid_width {
                let x_pos = self.offset.x + x as f32 * self.cell_size;
                painter.line_segment(
                    [
                        egui::pos2(x_pos, self.offset.y),
                        egui::pos2(
                            x_pos,
                            self.offset.y + self.grid_height as f32 * self.cell_size,
                        ),
                    ],
                    stroke,
                );
            }

            // Horizontal lines
            for y in 0..=self.grid_height {
                let y_pos = self.offset.y + y as f32 * self.cell_size;
                painter.line_segment(
                    [
                        egui::pos2(self.offset.x, y_pos),
                        egui::pos2(
                            self.offset.x + self.grid_width as f32 * self.cell_size,
                            y_pos,
                        ),
                    ],
                    stroke,
                );
            }
        }

        /// Get the total pixel size of the grid
        pub fn size(&self) -> egui::Vec2 {
            egui::vec2(
                self.grid_width as f32 * self.cell_size,
                self.grid_height as f32 * self.cell_size,
            )
        }
    }
}

/// Standard UI components for visualizations
pub mod ui {
    use super::*;

    /// Draw a standard control bar at the top of the visualization
    /// Shows play/pause status, speed, and keyboard hints
    pub fn control_bar(ui: &mut egui::Ui, playing: bool, speed: f32) {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(if playing { "▶ Playing" } else { "⏸ Paused" });
            ui.add_space(20.0);
            ui.label(format!("Speed: {:.1}x", speed));
            ui.add_space(20.0);
            ui.label("Space: Play/Pause");
            ui.add_space(20.0);
            ui.label("R: Reset");
        });
        ui.add_space(8.0);
    }

    /// Draw a centered control bar (useful for centered layouts)
    pub fn control_bar_centered(ui: &mut egui::Ui, playing: bool, speed: f32) {
        ui.vertical_centered(|ui| {
            control_bar(ui, playing, speed);
        });
    }

    /// Draw a minimal status line
    pub fn status_line(ui: &mut egui::Ui, status: &str) {
        ui.add_space(4.0);
        ui.centered_and_justified(|ui| {
            ui.label(status);
        });
        ui.add_space(4.0);
    }
}

/// Helper for animation control
pub mod animation {
    use super::*;

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

        /// Handle keyboard controls (Space to play/pause, R to reset)
        pub fn handle_keyboard(&mut self, ctx: &egui::Context) {
            ctx.input(|i| {
                if i.key_pressed(egui::Key::Space) {
                    self.playing = !self.playing;
                }
                if i.key_pressed(egui::Key::R) {
                    self.reset();
                    self.playing = false;
                }
            });
        }

        /// Draw minimal UI controls (just status)
        pub fn ui_minimal(&self, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                ui.label(if self.playing {
                    "▶ Playing"
                } else {
                    "⏸ Paused"
                });
                ui.separator();
                ui.label(format!("Speed: {:.1}x", self.speed));
                if self.total_frames > 0 {
                    ui.separator();
                    ui.label(format!(
                        "Frame: {}/{}",
                        self.current_frame, self.total_frames
                    ));
                }
            });
        }

        /// Draw full UI controls with buttons and sliders
        pub fn ui(&mut self, ui: &mut egui::Ui) {
            ui.horizontal(|ui| {
                if ui
                    .button(if self.playing {
                        "⏸ Pause"
                    } else {
                        "▶ Play"
                    })
                    .clicked()
                {
                    self.playing = !self.playing;
                }

                if ui.button("⏹ Reset").clicked() {
                    self.reset();
                    self.playing = false;
                }

                ui.separator();
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed, 0.1..=5.0));

                if self.total_frames > 0 {
                    ui.separator();
                    ui.label(format!(
                        "Frame: {}/{}",
                        self.current_frame, self.total_frames
                    ));
                }
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
