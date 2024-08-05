#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn handle(_ref: &PanicInfo) -> ! {
    loop {}
}

fn friend_call() -> isize {
    1
}

#[no_mangle]
fn _start() -> ! {
    loop {}
    //println!("Hello, world!");
}
