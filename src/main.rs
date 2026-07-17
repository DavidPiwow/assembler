use eframe::egui;

mod app;
use app::LC3App;

use crate::cpu::CPU;
use crate::parser::{scanner, syntax_tree};

mod cpu;
mod parser;
mod views;


fn main() {

    // window settings
    let window_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),  
        ..Default::default()    
    };

    // open the window
    let _ = eframe::run_native(
        "LC-3 Tools",         
        window_options, 

        // when the app open, egui runs it 
        Box::new(|_cc| Box::new(LC3App::new())),  // Box puts data on the heap
    );

}