use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_ms,
};
use log::*;

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
