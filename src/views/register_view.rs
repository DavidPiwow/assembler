use eframe::egui;
use crate::cpu::CPU;

pub fn draw(ui: &mut egui::Ui, cpu: &CPU) {

    ui.heading("Registers");
    let registers = cpu.view_registers();

    for i in 0..8 {
        let value = registers[i];

        // display: register name, value in hex, value as a signed number
        ui.label(format!(
            "R{}    x{:04X}    {}",
            i, value, value as i16
        ));
    }

    ui.separator();     // draw a dividing line between registers and PC

    // show the program counter
    let pc = cpu.view_pc();
    ui.label(format!(
        "PC    x{:04X}    {}", 
        pc, pc));
}