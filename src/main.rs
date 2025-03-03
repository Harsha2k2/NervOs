#![no_std]  // no std lib, we keep it barebones
#![no_main] // no main entry point, we do it our way
#![feature(abi_x86_interrupt)] // enable x86 interrupt feature

use core::panic::PanicInfo; // panic info for error handling

// entry point - this is where the magic starts
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to NervOS by Jatha");
    println!("--------------------------------");
    println!("Status: Booting...");

    // initializing idt
    println!("Initializing IDT...");
    interrupts::init_idt();
    println!("IDT initialized");

    // setting up the pics
    println!("Initializing PICS...");
    unsafe { 
        interrupts::PICS.lock().initialize();
        println!("PICS initialized");
    }

    // keyboard setup
    println!("Initializing keyboard...");
    keyboard::init_keyboard();
    println!("Keyboard initialized");

    // enabling interrupts
    println!("Enabling interrupts...");
    x86_64::instructions::interrupts::enable();
    println!("Interrupts enabled");

    println!("Status: Booted successfully!");
    println!("Memory: {} KB", 640);
    println!("--------------------------------");
    println!("System is ready - Press any key");
    println!("--------------------------------");

    // main loop
    println!("Entering main loop");
    loop {
        x86_64::instructions::interrupts::enable_and_hlt(); // keep it chill
    }
}

// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    println!("System halted."); // we done here
    loop {
        x86_64::instructions::hlt(); // just chillin'
    }
} 