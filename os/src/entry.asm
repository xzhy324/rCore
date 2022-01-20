# os/src/entry.asm
    .section .text.entry
    .globl _start
_start:
# 设定栈指针，并调用rust_main作为函数入口
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: