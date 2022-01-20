.cpu cortex-m4
.syntax unified
.thumb

// ARM semihosting operations
.equ SYS_WRITEC, 0x03
.equ SYS_WRITE0, 0x04
.equ SYS_READC,  0x07

.global __breakpoint
__breakpoint:
    bkpt
    bx lr

.global __context_switch
// r0 and r1 are holding the addresses of corresponding struct field psp
// * r0: *psp next process
// * r1: *psp from process (can be 0 when first switch is to pid0)
__context_switch:
    // Saves current process when r1 != 0
    mov r2, #0x0
    cmp r2, r1
    ITTT NE
    mrsne r2, psp
    stmdbne r2, {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    // Saves current psp in array
    stmne r1, {r2}

    // Loads new process
    ldm r0, {r0}
    ldmdb r0, {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    msr psp, r0
    isb

    bx lr

.global __syscall
__syscall:
    svc 0;
    bx lr

.global __sys_write0
__sys_write0:
    mov r1, r0
    mov r0, SYS_WRITE0
    bkpt 0xAB ;
    bx lr

.global __sys_writec
__sys_writec:
    mov r1, r0
    mov r0, SYS_WRITEC
    bkpt 0xAB ;
    bx lr

.global __sys_readc
__sys_readc:
    mov r1, #0x0
    mov r0, SYS_READC
    bkpt 0xAB ;
    bx lr

.global __get_r0
__get_r0:
    bx lr
    