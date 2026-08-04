use eframe::egui;

// Need David to check this code. RN, the I/O boxes are not talking to each other. 

// output = text the program has printed (shown read-only)
// input  = user replying to the program

pub fn draw(ui: &mut egui::Ui, output: &str, input: &mut String) -> bool {
    ui.heading("Output");

    // output area 
    egui::ScrollArea::vertical()
        .max_height(100.0)
        .stick_to_bottom(true)     // always keep the newest line in view
        .show(ui, |ui| {
            ui.monospace(output);
        });

    ui.separator();

    // input box
    let mut send_clicked = false;
    ui.horizontal(|ui| {
        ui.label("Input:");
        ui.text_edit_singleline(input);
        if ui.button("Send").clicked() {
            send_clicked = true;
        }
    });

    send_clicked
}
