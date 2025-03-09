use core::arch::asm;

const SSIZE: isize = 48; // try out 624?

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");
    loop {}
}
fn gt_switch(new: *const ThreadContext) {
    // Move what’s at the + 0x00 offset from the memory location that {compiler_chosen_general_purpose_register} points to to the rsp register.
    unsafe {
        asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new,
        )
    }
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;
        gt_switch(&mut ctx);
    }
}
