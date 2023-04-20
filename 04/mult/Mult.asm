// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// We want to compute x * y, which is the same as (y + ... + y) for x copies of
// y.
//
// So we'll loop x times, and add y to an accumulator once per loop iteration.

(START)
    @R2
    M=0

(LOOP)
    // If R0 equals 0, jump to end.
    @R0
    D=M
    @END
    D;JEQ

    // R2 += R1
    @R1
    D=M
    @R2
    M=D+M

    // R0 -= 1
    @R0
    M=M-1

    @LOOP
    0;JMP

(END)
