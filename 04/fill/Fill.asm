// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

(START)
    // R3 := screen
    // R4 := screen + 8K
    @SCREEN
    D=A
    @R3
    M=D
    @8192
    D=D+A
    @R4
    M=D

    // if a key is pressed, jump to BLACK
    @KBD
    D=M
    @BLACK
    D;JNE

(WHITE)
    // R5 := 16 zeros
    @R5
    M=0

    @LOOP
    0;JMP

(BLACK)
    // R5 := 16 ones
    @R5
    M=-1

    // fall through, to LOOP

(LOOP)
    // if R3 >= R4, break
    @R3
    D=M
    @R4
    D=D-M
    @END
    D;JGE

    // *R3 := 16 bits (pixels) of R5
    @R5
    D=M
    @R3
    A=M  // "dereference" R3
    M=D

    // R3 += 1
    @R3
    M=M+1

    @LOOP
    0;JMP

(END)
    // run forever
    @START
    0;JMP
