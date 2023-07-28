#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_item;
mod sbi;
#[macro_use]
mod console;
mod config;
pub mod loader;
mod logging;
mod sync;
pub mod syscall;
mod task;
mod timer;
pub mod trap;

#[path = "./boards/qemu.rs"]
mod qemu;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello World!");
    logging::init();
    trap::init();
    loader::load_app();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}
