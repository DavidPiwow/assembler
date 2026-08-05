

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