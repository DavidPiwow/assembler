.ORIG x3000 ; Program begins at x3000

LEA R0, prompt1 ; Semi-colons are used to make comments
PUTS ; This prints the string prompt1. It is an OS call

HALT

prompt1 .STRINGZ "Hello World!"
.END