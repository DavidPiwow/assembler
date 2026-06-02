use eframe::egui;

pub fn draw(ui: &mut egui::Ui, source_text: &str) {
    ui.heading("Program");
    ui.monospace(source_text); // monospace: character is the same widt
} 