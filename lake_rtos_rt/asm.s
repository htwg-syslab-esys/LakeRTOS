.cpu cortex-m4
.syntax unified
.thumb

.global __breakpoint
__breakpoint:
    bkpt
    bx lr

.global __context_switch
__context_switch:
    svc 0
    bx lr

.type SVCall, %function
.global SVCall
SVCall: 
    mov r2, #0xfffffffd
    cmp r2, lr
    ITTT EQ
    mrseq r2, psp
    stmdbeq r2, {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    stmeq r1, {r2}

    ldmdb r0, {r4, r5, r6, r7, r8, r9, r10, r11, lr}
    msr psp, r0
    isb

    bx lr