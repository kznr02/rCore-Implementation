use crate::{println, sbi::shutdown};
use core::panic::PanicInfo;
use log::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        error!(
            "[kernel] Panickeda at {}:{} {}",
            location.file(),
            location.line(),
            _info.message().unwrap()
        );
    } else {
        error!("[kernel] Panicked: {}", _info.message().unwrap())
    }
    shutdown(true)
}
