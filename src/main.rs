#![no_std]      // 不链接rust标准库
#![no_main]     // 禁用main入口

use core::panic::PanicInfo;

/// 在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World";

/// 被引导程序调用
#[no_mangle]    // 不重整函数名
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xa;   // 0xa为绿色
        }
    }

    loop {}
}
