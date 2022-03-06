mod headlines;
use eframe::{run_native, NativeOptions};
use egui::{Ui, Vec2, Separator};
use headlines::{Headlines, PADDING};

fn main() {
    let app = Headlines::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));

    run_native(Box::new(app), win_option);
}

fn render_footer(ui: &mut Ui) {}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sept = Separator::default().spacing(20.);
}
