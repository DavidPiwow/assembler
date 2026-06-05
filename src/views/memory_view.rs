use eframe::egui::{self, RichText, Color32};
use crate::cpu::CPU;

pub fn draw(ui: &mut egui::Ui, cpu: &CPU) {
    ui.heading("Memory");
    let pc: usize = cpu.view_pc() as usize;

    // memory window
    let start: usize = pc.saturating_sub(5);         // start 5 addresses before PC
    let end = (pc + 20).min(0xFFFE);         // end 20 addresses after PC (caps at 2^16)

    // get that slice of memory from the CPU
    let memory = cpu.view_memory_slice(start, end);

    ui.monospace(RichText::new("   Address    Hex      Dec").color(Color32::DARK_GRAY));

    for i in 0..memory.len() {
        let address = start + i;   // the memory address of this row
        let value = memory[i];     // the value stored at this address

        // display: arrow, address, value in hex, value as a number
        match address == pc {
            true => {
                ui.monospace(RichText::from(format!(
                    "▶  x{:04X}    x{:04X}    {}",
                    address, value, value
                )).strong());
            }, 
            false => {
                ui.monospace(format!(
                    "    x{:04X}    x{:04X}    {}",
                    address, value, value
                ));
            }
        }
    }
}