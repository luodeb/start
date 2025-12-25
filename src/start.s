// startup.S
.section .text.boot, "ax"
.global _start

_start:
    msr daifset, #2   // 关闭所有中断

    // 设置栈指针（使用链接脚本定义的栈顶）
    ldr x0, =boot_stack_top
    mov sp, x0

    // 调用 Rust 的 kernel_main 函数
    bl kernel_main

    // 死循环，防止返回
1:  wfe
    b 1b

// 栈空间（放在 .bss.stack 段，与链接脚本对应）
.section .bss.stack, "aw", @nobits
.align 12
.global boot_stack
boot_stack:
    .skip 0x10000  // 64KB 的栈空间
.global boot_stack_top
boot_stack_top: