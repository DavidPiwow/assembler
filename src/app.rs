use eframe::egui;
use crate::cpu::CPU;
use crate::parser::{scanner, syntax_tree};

pub struct LC3App {
    pub cpu: CPU,                            // from David's
    pub source_text: String,                // the assembly code text
    pub running: bool,          
    pub assembled: bool,                 // added this since assembling has a seperate button
    pub error_message: Option<String>,  // show error messages
}

impl LC3App {
    pub fn new() -> Self {
        let cpu = CPU::default();    
        
        LC3App {
            cpu,
            source_text: String::new(),        // user writes their own assembly
            running: false,
            assembled: false,
            error_message: None,
        }
    }
    
    // try to assemble the code
    fn try_assemble(&mut self) -> bool {
        self.error_message = None;
        
        // check if there's code in the editor
        if self.source_text.trim().is_empty() {
            self.error_message = Some("No code to run".to_string());
            return false;
        }
        
        let tokens: Vec<scanner::Token> = scanner::tokenize(&self.source_text);
        
        // parser taken tokens ->  program
        let program = match syntax_tree::scan_sequence(tokens) {
            Ok(prog) => prog,
            Err(e) => {
                self.error_message = Some(format!("Error: {}", e));
                return false;
            }
        };
        
        // calls to_binary() on program -> machine code
        let mut program = program;
        program.to_binary();
        let binary = program.get_binary();
        
        // calls set_program() on machine code -> loaded in CPU memory
        if !binary.is_empty() {
            self.cpu.set_program(binary);
            self.assembled = true;
            return true;                   // assembled at this point
       
        } else {
            // handles if user types only non-executable things
            self.error_message = Some("No instructions generated".to_string());
            return false;
        }
    }
}

impl eframe::App for LC3App {

    // main loop, egui calls this every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.running {
            self.cpu.step();
            ctx.request_repaint();         // keep updating the display
        }

        // add assemble, run/pause, step, rest at the top
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("LC-3 Tools");

                ui.separator();

                // ASSEMBLE button
                if ui.button("Assemble").clicked() {
                    self.try_assemble();
                }

                ui.separator();

                // STEP button (increment by one instruction)
                if ui.button("Step").clicked() {
                    if self.assembled {
                        self.cpu.step();
                    } else {
                        self.error_message = Some("Assemble first".to_string());
                    }
                }

                // RUN button
                if !self.running {
                    if ui.button("Run").clicked() {
                        if self.assembled {
                            self.running = true;
                        } else {
                            self.error_message = Some(" Assemble first".to_string());
                        }
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
                    self.cpu = CPU::default();
                    self.running = false;
                    self.assembled = false;          // need to reassemble after reset
                }
                
                // show error messages
                if let Some(ref error) = self.error_message {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, error);
                }
            });
        });

        // LEFT widget (source code editor)
        egui::SidePanel::left("source_panel").min_width(250.0).show(ctx, |ui| {
            crate::views::source_view::draw_editor(ui, &mut self.source_text);
        });

        // RIGHT widget (registers and PC)
        egui::SidePanel::right("register_panel").min_width(220.0).show(ctx, |ui| {
            crate::views::register_view::draw(ui, &self.cpu);
        });

        // CENTER widget (memory view)
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::views::memory_view::draw(ui, &self.cpu);
        });
    }
}