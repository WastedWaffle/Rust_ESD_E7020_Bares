//! bare0.rs
//!
//! Simple bare metal application
//! What it covers:
//! - constants
//! - global (static) variables
//! - checked vs. wrapping arithmetics
//! - safe and unsafe code
//! - making a safe API
//! - assertions
//! - panic handling

// build without the Rust standard library
#![no_std]
// no standard main, we declare main using [entry]
#![no_main]

// Panic handler, for textual output using semihosting
use panic_semihosting as _;


// import entry point
use cortex_m_rt::entry;

// a constant (cannot be changed at run-time)

const X_INIT: u32 = 10;
// const X_INIT: u32 = core::u32::MAX;

// global mutable variables (changed using unsafe code)
static mut X: u32 = X_INIT;
static mut Y: u32 = 0;

fn write_u32(v: &mut u32, x:u32){
    unsafe{
        *v = x  
    }
}

fn read_u32(v:&u32)-> u32{
    unsafe{*v}
}

#[entry]
fn main() -> ! {
    // local mutable variable (changed in safe code)

        let mut x = read_u32(&X);

        loop {

            x =x.wrapping_add(1); // <- place breakpoint here (3)
            
            write_u32(&mut X, read_u32(&X).wrapping_add(1));
            write_u32(&mut Y, read_u32(&X));
            
            
            assert!(x == read_u32(&X) && read_u32(&X) == read_u32(&Y));
        
        
        
    }
}

// Here we assume you are using `vscode` with `cortex-debug`.
//
// 0. Compile/build and run the example in debug (dev) mode.
//
//    > cargo run --example bare0
//    (or use vscode)
//
// 1. Run the program in the debugger, let the program run for a while and
//    then press pause.
//
//    Look under Variables/Local what do you find.
//
//    ** your answer here **
//      Ans: I find that the x = 6950998 and no sign of Y
//          By using "let _ = core::ptr::read_volatile(&Y);" we are able to read the value of Y 
//          and in that case Y = X
//
//    In the Expressions (WATCH -vscode) view add X and Y
//    what do you find
//
//    ** your answer here **
//      Ans: I find that    X: 6950998
//                  &       Y:<optimized out>  
//          By using "let _ = core::ptr::read_volatile(&Y);" we are able to read the value of Y 
//          and in that case Y = X
//
//    Step through one complete iteration of the loop
//    and see how the (Local) Variables are updated
//    can you foresee what will eventually happen?
//
// 	  ** place your answer here **
//          It will eventually owerflow
//
//    Commit your answers (bare0_1)
//
//_______________________________________________________________________________________________
// 2. Alter the constant X_INIT so that `x += 1` directly causes `x` to wrap.
// 	  What happens when `x` wraps
//    (Hint, look under OUTPUT/Adopter Output to see the `openocd` output.)
//
//    ** your answer here **
//      It !panics due: rust_begin_unwind (info=0x20007fd0)
//      panicked at 'assertion failed: x == X && X == Y'
//      It will say x != X
//
//    Commit your answers (bare0_2)
//
//_______________________________________________________________________________________________
// 3. Place a breakpoint at `x += 1`
//
//    Change (both) += operations to use wrapping_add
//    load and run the program, what happens
//    ** your answer here **
//      the variables wraps around now without any !panics. It comes back to the breakpoint
//
//
//    Now continue execution, what happens
//    ** your answer here **
//      It will be an infinit loop i.e. the program will continue to run forever.
//
//
//    Commit your answers (bare0_3)
//
//    (If the program did not succeed back to the breakpoint
//    you have some fault in the program and go back to 3.)
//
//_______________________________________________________________________________________________
// 4. Change the assertion to `assert!(x == X && X == Y + 1)`, what happens?
//
//    ** place your answer here **
//      It panicked at 'assertion failed: x == X && X == Y + 1'
//      Because (X != Y+1) while (X = Y)
//
//
//    Commit your answers (bare0_4)
//
//_______________________________________________________________________________________________
// 5. Remove the assertion and implement "safe" functions for
//    reading and writing X and Y
//    e.g. read_x, read_y, write_x, write_y
//
//    Rewrite the program to use ONLY "safe" code besides the
//    read/write functions (which are internally "unsafe")
//
//    Commit your solution (bare0_5)
//
//_______________________________________________________________________________________________
// 6. *Optional
//    Implement a read_u32/write_u32, taking a reference to a
//    "static" variable
//
//    Rewrite the program to use this abstraction instead of "read_x", etc.
//
//    Commit your solution (bare0_6)
//    
//_______________________________________________________________________________________________
