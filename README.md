# M43Lang

## Documentation

START $d: starts program onto direction $d
GO $i: puts pointer at $i
RIGHT $i: movevs pointer $i to the right
LEFT $i: movevs pointer $i to the left
SET $v: sets $v to pointer position
ADD $v: add $v to ponter position
MUL $v: multiplies $v onto pointer position
DIV $v: divides pointer position by $v
SUB $v: subtract $v from ponter position
REDIRECT $d: changes program's direction to $d
CONDITIONAL $d1 $d2: if current value is 0, goes direction $d2, else $d1
COMP: 
PRINT: prints value pointer points to
DISPLAY: prints value pointer points to in ascii
INPUT: sets current value to value input by user
END: ends program