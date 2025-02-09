// this will make sure that the rust executable is not linked to the rust standard library during the compiler linker stage
// at the top of the problem
#![no_std]
#![no_main]

// PanicInfo contains the file and line no. where panic occurred.
use core::panic::PanicInfo;

// RFC 1513: allows custom panic configuration at compile time...
// panic_handler implementation: this function is called on panic
#[panic_handler]
// _info => ensures that this argument will remain unused, without any warning by the compiler...
// Diverging function: function that do not return, it diverges(! is called never type),
// once the panic occurs, current thread of execution stops and program crashes, for it never returns.
// it diverges instead from it's usual flow, that's why the name.
// you can see the backtrace of error via RUST_BACKTRACE=1 or full ./program 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// using main requires the standard library, 
// instead use #[no_main]
// fn main() {
// }
//instead use this:
 #[no_mangle]  // this is related to how rust interacts with other languages
// normally rust mangles(changes) the function names to include extra information
// no_mangle ensures that the function name is not changed, at compile time.
// extern means this function can be called from outside of my rust code...
// here extern "C" is used, meaning the code will be called from c code.
// pub => made the function public, so other code can use 
/*
// Regular Rust function - name will be mangled
fn normal_function() {
    println!("Hello!");
}

// Unmangled function that can be called from C code
#[no_mangle]
pub extern "C" fn unmangled_function() {
    println!("Hello!");
}
*/
pub extern "C" fn _start() -> ! {  // Platform-specific entry point
    loop {} // Program must not return, hence the infinite loop
}