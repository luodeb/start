#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

// 引入汇编启动代码
global_asm!(include_str!("start.s"));

// UART0 基地址 (QEMU virt 机器)
// const UART0_BASE: usize = 0x18002000;
const UART0_BASE: usize = 0x09000000;

/// 向 UART 写入一个字符
fn uart_putc(c: u8) {
    unsafe {
        let uart = UART0_BASE as *mut u8;
        core::ptr::write_volatile(uart, c);
    }
}

/// 打印字符串到 UART
fn uart_puts(s: &str) {
    for c in s.bytes() {
        uart_putc(c);
    }
}

/// 内核主函数，由汇编启动代码调用
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    // 清空 BSS 段
    clear_bss();
    
    uart_puts("Hello, Bare Metal World!\n");
    
    loop {
        unsafe { core::arch::asm!("wfe") };
    }
}

/// 清空 BSS 段
fn clear_bss() {
    unsafe extern "C" {
        unsafe static mut sbss: u64;
        unsafe static mut ebss: u64;
    }
    unsafe {
        let start = &raw mut sbss as *mut u64 as usize;
        let end = &raw mut ebss as *mut u64 as usize;
        for addr in (start..end).step_by(8) {
            (addr as *mut u64).write_volatile(0);
        }
    }
}

/// Panic 处理函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_puts("\n!!! PANIC !!!\n");
    if let Some(location) = info.location() {
        uart_puts("File: ");
        uart_puts(location.file());
        uart_puts("\n");
    }
    loop {
        unsafe { core::arch::asm!("wfe") };
    }
}


