use eframe::egui;
use crate::cpu::CPU;
use crate::parser::scanner::TokenError;
use crate::parser::{scanner, syntax_tree};

pub struct LC3App {
    pub cpu: CPU,                            // from David's
    pub source_text: String,                // the assembly code text
    pub running: bool,          
    pub assembled: bool,                 // added this since assembling has a seperate button
    pub error_message: Option<String>,  // show error messages
}


const TEST_OS: &str = ".ORIG x0000
; the TRAP vector table
    .FILL 0    ; x00
    .FILL 0    ; x01
    .FILL 0    ; x02
    .FILL 0    ; x03
    .FILL 0    ; x04
    .FILL 0    ; x05
    .FILL 0    ; x06
    .FILL 0    ; x07
    .FILL 0    ; x08
    .FILL 0    ; x09
    .FILL 0    ; x0A
    .FILL 0    ; x0B
    .FILL 0    ; x0C
    .FILL 0    ; x0D
    .FILL 0    ; x0E
    .FILL 0    ; x0F
    .FILL 0    ; x10
    .FILL 0    ; x11
    .FILL 0    ; x12
    .FILL 0    ; x13
    .FILL 0    ; x14
    .FILL 0    ; x15
    .FILL 0    ; x16
    .FILL 0    ; x17
    .FILL 0    ; x18
    .FILL 0    ; x19
    .FILL 0    ; x1A
    .FILL 0    ; x1B
    .FILL 0    ; x1C
    .FILL 0    ; x1D
    .FILL 0    ; x1E
    .FILL 0    ; x1F
    .FILL TRAP_GETC   ; x20
    .FILL TRAP_OUT    ; x21
    .FILL TRAP_PUTS   ; x22
    .FILL TRAP_IN     ; x23
    .FILL 0  ; x24
    .FILL 0   ; x25


TRAP_GETC
    LDI R0, OS_KBSR        ; wait for a keystroke (cpu set it to -1)
    BRzp TRAP_GETC
    AND R0, R0, #0
    STI R0, OS_KBSR        ; clear the bit to signal it was read
    LDI R0, OS_KBDR        ; read it and return
    RTI

OS_KBSR    .FILL xFE00     ; the cpu has the ability to set this to -1, the os can only clear it
OS_KBDR    .FILL xFE02

TRAP_OUT
    STI R0, OS_DDR        ; write the character and return
    RTI

OS_DSR     .FILL xFE04
OS_DDR     .FILL xFE06
OS_SP      .FILL x3000


TRAP_PUTS
    ADD R1, R0, #0        ; move string pointer (R0) into R1
TRAP_PUTS_LOOP
    LDR R0, R1, #0        ; write characters in string using OUT
    BRz TRAP_PUTS_DONE
    OUT
    ADD R1, R1, #1
    BRnzp TRAP_PUTS_LOOP
TRAP_PUTS_DONE
    RTI

    

TRAP_IN
    LEA R0, TRAP_IN_MSG    ; prompt for input
    PUTS
    GETC                   ; read a character
    OUT                    ; echo back to monitor
    ADD R6, R6, #-1
    STR R0, R6, #0         ; save the character
    AND R0, R0, #0         ; write a linefeed, too
    ADD R0, R0, #10
    OUT
    LDR R0, R6, #0         ; restore the character
    ADD R6, R6, #1
    RTI

TRAP_IN_MSG    .STRINGZ \"\nInput a character> \"
.END

";

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
    fn try_assemble(&mut self) -> Result<(), TokenError> {
        // check if there's code in the editor
        if self.source_text.trim().is_empty() {
            return Err(TokenError::EmptyProgram);
        }
        
        let tokens: Vec<scanner::Token> = scanner::tokenize(&self.source_text)?;

        // parser taken tokens ->  program
        let os_tokens = scanner::tokenize(TEST_OS)?;
        let mut os_program = syntax_tree::scan_sequence(os_tokens)?;
        os_program.to_binary()?;
        let os_binary = os_program.get_binary();

        self.cpu.set_program(0, os_binary);

        let mut program =  syntax_tree::scan_sequence(tokens)?;
        // calls to_binary() on program -> machine code
        program.to_binary()?;

        let binary = program.get_binary();
        let start = program.get_start();

        // calls set_program() on machine code -> loaded in CPU memory
        if !binary.is_empty() {
            self.cpu.set_program(start, binary);
            self.assembled = true;
            return Result::Ok(());                   // assembled at this point
        } else {
            // handles if user types only non-executable things
            return Err(TokenError::EmptyProgram);
        }
    }
}

impl eframe::App for LC3App {

    // main loop, egui calls this every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.running && self.cpu.view_mcr() != 0 {
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
                    self.error_message = match self.try_assemble() {
                        Result::Ok(_) => None,
                        Err(error) => {
                            Some(error.to_string())
                        }
                    };
                }

                ui.separator();

                // STEP button (increment by one instruction)
                if ui.button("Step").clicked() {
                    if self.assembled && self.cpu.view_mcr() != 0 {
                        self.cpu.step();
                    } else {
                        self.error_message = Some("Assemble first".to_string());
                    }
                }

                // RUN button
                if !self.running {
                    if ui.button("Run").clicked() {
                        if self.assembled && self.cpu.view_mcr() != 0 {
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