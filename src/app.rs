use eframe::egui;
use crate::cpu::CPU;

pub struct LC3App {
    pub cpu: CPU,                // from David's backend
    pub source_text: String,    // the assembly program text (will be on the left side of the screen) 
    pub running: bool,    
}


impl LC3App {
    pub fn new() -> Self {
        let mut cpu = CPU::default();    // create a blank CPU
        let test_program = [0x1023u16, 0x1265, 0x1642, 0x98FF];  // hardcoded machine code (placeholder)
        cpu.set_program(&test_program);                                   // hardcoded assembly text (placeholder)

        // hardcoded source text (placeholder)
        LC3App {
            cpu,
            source_text: String::from("ADD R0, R0, R3.                  
                                    \nADD R1, R1, R5    
                                    \nNOT R4, R3"),
            running: false,
        }
    }
}


impl eframe::App for LC3App {

    // main loop, egui calls this every window
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.running {
            self.cpu.step();                     // if the program is running, step the CPU once every current window
            ctx.request_repaint();              // keeps drawing a new window during every iteration
        }

        // put the step, run, pause on top
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("LC-3 Tools");

                // STEP button
                if ui.button("Step").clicked() {
                    self.cpu.step(); // 
                }

                // RUN button
                if !self.running {
                    if ui.button("Run").clicked() {
                        self.running = true;
                    }
                }

                // PAUSE button 
                if self.running {
                    if ui.button("Pause").clicked() {
                        self.running = false;
                    }
                }

                // RESET button 
                if ui.button("Reset").clicked() {
                    self.cpu = CPU::default(); // default() creates a blank struct with all values reset to zero
                    self.running = false; 
                }
            });
        });

        // LEFT PANEL (source code viewer)
        egui::SidePanel::left("source_panel").min_width(250.0).show(ctx, |ui| {
            crate::views::source_view::draw(ui, &self.source_text);
        });

        // RIGHT PANEL (registers and PC)
        egui::SidePanel::right("register_panel").min_width(220.0).show(ctx, |ui| {
            crate::views::register_view::draw(ui, &self.cpu);
        });

        // CENTER PANEL (memory view)
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::views::memory_view::draw(ui, &self.cpu);
        });
    }
}