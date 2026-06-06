LC-3 Tools Rewrite using Rust

# Code Explanation
###  Parser 
  - scanner.rs takes in a text file and converts it into a list of nodes
  - node.rs contains all Node structs
  - syntax_tree.rs builds the syntax tree that represents the program
  - program.rs contains the code to convert from syntax tree to binary

###  App 
  - Under views, memory_view.rs, register_view.rs, and source_view.rs all draw their respective views to screen
  - app.rs contains the main loop logic and runs the CPU instance.

### CPU
  - Contains all the code for running the LC-3 instructions based on binary
  - It was written to emulate how the LC-3 instruction cycle works using things such as MAR/MDR




# Current Missing Features
  - Missing interrupts and RTI (return from interrupt) 
  - No syntax highlighting
  - Multiple program view
  - No way to convert from binary to instruction


## Credits
  David & Safa wrote the CPU/assembler
  Sneha & Daniel wrote the app logic
