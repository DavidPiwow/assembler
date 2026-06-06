ADD R4, R4, #3 ; 'A
ADD R5, R5, #5 ; 'B'
AND R6, R6, #0 ; 'C'

LOOP
ADD R6, R6, R4  ; 'C' += 'A'

ADD R5, R5, #-1 ; while 'B' > 0

BRp LOOP
STR R6, R3, #2
HALT
