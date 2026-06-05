.ORIG x3000     ; directive: put code at the start of user memory

LD R3, BaseAddr ; R3 now contains the base address x3100

AND R4, R4, #0 ; 'A
AND R5, R5, #0 ; 'B'
AND R6, R6, #0 ; 'C'
LDR R4, R3, #0 ; load 'A'
LDR R5, R3, #1 ; load 'B'
LOOP
ADD R6, R6, R4  ; 'C' += 'A'

ADD R5, R5, #-1 ; while 'B' > 0

BRp LOOP

STR R6, R3, #2

HALT

; ************ DATA ************

BaseAddr        .FILL x3100

        .END   