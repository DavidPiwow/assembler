LC-3 Tools Rewrite using Rust

# Code Explanation
###  Parser 
  - scanner.rs takes in a text file and converts it into a list of nodes
  - node.rs contains all Node structs
  - syntax_tree.rs builds the syntax tree that represents the program
  - program.rs contains the code to convert from syntax tree to binary

  Note: The parser should not try to modify the program given, only read it

###  App 
  - Under views, memory_view.rs, register_view.rs, console_view.rs, and source_view.rs all draw their respective views to screen
  - app.rs contains the main loop logic and runs the CPU instance.

### CPU
  - Contains all the code for running the LC-3 instructions based on binary
  - It was written to emulate how the LC-3 instruction cycle works using things such as MAR/MDR

### Documentation
  - Run `cargo doc --no-deps` to generate the documentation file for the project. (You can also add `--open` to open it automatically)




# Current Missing Features
  - Missing interrupts
  - No syntax highlighting
  - Multiple program view
  - No way to convert from binary to instruction
  - Escape sequences (\n, \t, ..etc) need to be dealt with. This is an issue with how the egui TextEdit deals with strings.


## Credits
  David & Safa (@saftah) - CPU and assembler  
  sneha (@snh-roy) - Frontend GUI implementation, assembler integration  
  Matthew (@matt-s3190) - Text editor widget for program input
