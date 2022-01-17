.cpu cortex-m4
.syntax unified
.thumb

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
    