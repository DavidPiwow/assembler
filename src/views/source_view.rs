use eframe::egui;

pub fn draw_editor(ui: &mut egui::Ui, source_text: &mut String) {
    ui.heading("Program");
    
    //  egui creates a text edit widget
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add(
            egui::TextEdit::multiline(source_text)
                .font(egui::TextStyle::Monospace)
                .desired_width(f32::INFINITY)
                .desired_rows(30)
        );
    });
} 