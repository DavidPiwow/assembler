;
; LC3 Tutor - basicinputoutput.asm
;
; Prints a prompt to the user and gets the user input.
;



    .ORIG x0000
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
    .FILL 0   ; x20
    .FILL TRAP_OUT    ; x21
    .FILL TRAP_PUTS   ; x22
    .FILL 0     ; x23
    .FILL 0  ; x24
    .FILL 0   ; x25

TRAP_OUT
    LD R6, OS_SP         ; actually load the stack pointer so its not just garbage 

    ADD R6, R6, #-1
    STR R1, R6, #0        ; save R1

    ADD R6, R6, #-1
    STR R2, R6, #0        ; save R2
TRAP_OUT_WAIT
    LDI R1, OS_DSR        ; wait for the display to be ready (same deal as text input)
    BRzp TRAP_OUT_WAIT

    STI R0, OS_DDR        ; write the character and return

    LDR R2, R6, #0        ; restore R2
    ADD R6, R6, #1
    LDR R1, R6, #0        ; restore R1
    ADD R6, R6, #1
    RTI

OS_DSR     .FILL xFE04
OS_DDR     .FILL xFE06
OS_SP      .FILL x3000


TRAP_PUTS

    LD R6, OS_SP         ; actually load the stack pointer so its not just garbage 
    ADD R6, R6, #-1       ; save R0 and R1
    STR R0, R6, #0
    ADD R6, R6, #-1
    STR R1, R6, #0
    ADD R1, R0, #0        ; move string pointer (R0) into R1
TRAP_PUTS_LOOP
    LDR R0, R1, #0        ; write characters in string using OUT
    BRz TRAP_PUTS_DONE
    OUT
    ADD R1, R1, #1
    BRnzp TRAP_PUTS_LOOP
TRAP_PUTS_DONE
    LDR R1, R6, #0         ; restore R0 and R1
    ADD R6, R6, #1
    LDR R0, R6, #0
    ADD R6, R6, #1
    RTI



.END

.ORIG x3000 ; Program begins at x3000.

LEA R0, prompt1 ; Semi-colons are used to create comments.
PUTS ; This prints the string prompt1. It is an OS call.

GETC ; Gets one character from the user and puts into R0.
OUT ; Echo user input.

; Without LD/ST we must use multiple ADD/AND/NOT.
AND R1, R1, #0 ; Clear R1.
ADD R1, R1, #15
ADD R1, R1, #15
ADD R1, R1, #2 ; #32 or x20 is used to convert upper to lower case.

; We now need to OR R0 with R1 to get the desired result.
; We can use DeMorgans principle to OR using AND and NOT.
NOT R0, R0
NOT R1, R1
AND R1, R0, R1
NOT R1, R1 ; The lower case letter is in R1.

LEA R0, prompt2
PUTS

ADD R0, R1, #0 ; Put R1 in R0.
OUT

HALT ; The program is done, all objectives met.

prompt1 .STRINGZ "Input an upper case letter: "
prompt2 .STRINGZ "\nThe lower case letter is: "

.END ; Indicates the end of the assembly file.