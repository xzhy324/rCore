#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
//use log::{self,info, warn, error};

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod sbi;
mod sync;
mod syscall;
mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    //在内核加载时完成bss段的清零工作
    clear_bss();

    extern "C" {fn stext();fn etext();}
    extern "C" {fn srodata(); fn erodata();}
    extern "C" {fn edata();fn sdata();}
    extern "C" {fn skernel(); fn ekernel();}


    debug!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    debug!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    debug!("load range : [{:#x}, {:#x}]\n", skernel as usize, ekernel as usize);

    trap::init();
    batch::init();
    batch::run_next_app();

}




//完成对内核Block Started by Symbol（BSS段）的初始化工作，这段一般用来保存未初始化的全局变量，在本实验中也是堆栈存放的位置
fn clear_bss() {
    /*extern “C” 可以引用一个外部的 C 函数接口（这意味着调用它的时候要遵从目标平台的 C 语言调用规范）。
    但我们这里只是引用位置标志并将其转成 usize 获取它的地址。由此可以知道 .bss 段两端的地址
    */
    extern "C" {
        // 由链接脚本给出
        fn sbss();
        fn ebss();
    }
    //rust的迭代器与闭包语法
    (sbss as usize..ebss as usize).for_each (|a| {
        //使用裸指针需要用unsafe块框住
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}