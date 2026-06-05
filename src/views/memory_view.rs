use eframe::egui;
use crate::cpu::CPU;

pub fn draw(ui: &mut egui::Ui, cpu: &CPU) {
    ui.heading("Memory");
    let pc: usize = cpu.view_pc() as usize;

    // memory window
    let start: usize = pc.saturating_sub(5);         // start 5 addresses before PC
    let end = (pc + 20).min(0xFFFE);         // end 20 addresses after PC (caps at (2^16)-1)

    // get that slice of memory from the CPU
    let memory = cpu.view_memory_slice(start, end);

    for i in 0..memory.len() {
        let address = start + i;   // the memory address of this row
        let value = memory[i];     // the value stored at this address

        // put an arrow on the row that the PC is pointing at
        let arrow;
        if address == pc {
            arrow = "▶";
        } else {
            arrow = "  ";
        }

        // display: arrow, address, value in hex, value as a number
        ui.monospace(format!(
            "{}  x{:04X}    x{:04X}    {}",
            arrow, address, value, value
        ));
    }
}