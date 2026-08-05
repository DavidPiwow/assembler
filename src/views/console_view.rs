use eframe::egui;

// output    = text the program has printed (shown read-only)
// input     = user replying to the program
// key_ready = true when the CPU hit GETC and is waiting from the user
//             (cpu.is_key_ready()); only then do we show the input row

pub fn draw(ui: &mut egui::Ui, output: &str, input: &mut String, key_ready: bool) -> bool {
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
    let mut send_clicked = false; // only if the program is waiting to hear from the user
    if key_ready {
        ui.horizontal(|ui| {
            ui.label("Input:");
            let response = ui.text_edit_singleline(input);
            let entered = response.lost_focus()     // if the user prefers to just hit the Enter key
                && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button("Send").clicked() || entered {
                send_clicked = true;
            }
        });
    } else {
        ui.weak("Program is not waiting for input.");
    }

    send_clicked
}
