use core::mem::transmute;

use core::failure;
use core::fmt;

use platform::io;
use platform::cpu::mmu::Page;
use platform::cpu::Context;

use kernel::syscall;

#[repr(u8)]
pub enum Fault {
    DivideError = 0,

    NMI = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundExceeded = 5,
    InvalidOpcode = 6,
    NoMathCoprocessor = 7,
    DoubleFault = 8,
    CoprocessorSegmentOverun = 9,
    InvalidTss = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtection = 13,
    PageFault = 14,

    FloatingPointError = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SimdFpException = 19,

    SystemCall = 21
}

static Exceptions: &'static [&'static str] = &[
    "Divide-by-zero Error",
    "Debug",
    "Non-maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device Not Available",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Invalid TSS",
    "Segment Not Present",
    "Stack-Segment Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",                         // 15
    "x87 Floating-Point Exception",
    "Alignment Check",
    "Machine Check",
    "SIMD Floating-Point Exception",
    "Virtualization Exception",
    "System Call"                       // 21
];

// TODO respect destructors
#[lang="begin_unwind"]
unsafe extern "C" fn begin_unwind(fmt: &fmt::Arguments, file: &str, line: uint) -> ! {
    asm!("hlt");
    loop { }; // for divergence check
}

#[no_split_stack]
#[inline(never)]
unsafe fn blue_screen(stack: &Context) {
    io::puts("Exception ");
    io::puts(Exceptions[stack.int_no as uint]);
    asm!("hlt");
}

#[no_split_stack]
#[inline(never)]
pub unsafe fn exception_handler() -> unsafe extern "C" fn() {
    asm!("jmp skip_exception_handler
      exception_handler_asm:"
        :::: "volatile", "intel");

    // Points to the data on the stack
    let stack_ptr = Context::save();

    let fault: Fault = transmute(stack_ptr.int_no as u8);
    match fault {
        PageFault => {
            let cr2: uint;
            asm!("mov %cr2, %eax" : "={eax}"(cr2));
            println!("Accessed {0:x} from {1:x}", cr2, stack_ptr.call_stack.eip);
            blue_screen(stack_ptr);
        }
        Breakpoint => {
            asm!("debug:" :::: "volatile");
        }
        SystemCall => {
            stack_ptr.eax = syscall::syscall(
                stack_ptr.eax as uint,
                stack_ptr.edi as int,
                stack_ptr.esi as int,
                stack_ptr.edx as int,
                stack_ptr.ecx as int,
                0i,
                0i) as u32;
        }
        _ => blue_screen(stack_ptr)
    }

    Context::restore();

    asm!("skip_exception_handler:"
        :::: "volatile", "intel");

    extern { fn exception_handler_asm(); }
    exception_handler_asm
}
