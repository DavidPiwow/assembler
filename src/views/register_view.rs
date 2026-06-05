use eframe::egui;
use crate::cpu::CPU;

pub fn draw(ui: &mut egui::Ui, cpu: &CPU) {
    ui.heading("Registers");
    let registers = cpu.view_registers();

    for i in 0..8 {
        let value = registers[i];

        ui.label(format!(
            "R{}    x{:04X}    {}",                     // display: {register name}, {hex value}, {signed decimal number}
            i, value, value as i16
        ));
    }

    ui.separator();    

    // show the pc
    let pc = cpu.view_pc();
    ui.label(format!(
        "PC    x{:04X}    {}", 
        pc, pc));
}