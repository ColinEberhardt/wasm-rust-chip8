use cpu::Cpu;
use display::Display;
use keypad::Keypad;
use rand::{ComplementaryMultiplyWithCarryGen, CMWC_CYCLE};

static mut CPU: Cpu = Cpu::new_const();

#[no_mangle]
pub fn reset() {
    unsafe {
        CPU.reset();
    }
}

#[no_mangle]
pub fn get_memory() -> &'static [u8; 4096] {
    unsafe {
        &CPU.memory
    }
}

#[no_mangle]
pub fn get_display() -> &'static [u8; 2048] {
    unsafe {
        &CPU.display.memory
    }
}

#[no_mangle]
pub fn key_down(i: u8) {
    unsafe {
        CPU.keypad.key_down(i);
    }
}

#[no_mangle]
pub fn key_up(i: u8) {
    unsafe {
        CPU.keypad.key_up(i);
    }
}

#[no_mangle]
pub fn get_register_v() -> &'static [u8; 16] {
    unsafe {
        &CPU.v
    }
}

#[no_mangle]
pub fn get_register_i() -> u16 {
    unsafe {
        CPU.i
    }
}

#[no_mangle]
pub fn get_register_pc() -> u16 {
    unsafe {
        CPU.pc
    }
}

#[no_mangle]
pub fn execute_cycle() {
    unsafe {
        CPU.execute_cycle();
    }
}

#[no_mangle]
pub fn decrement_timers() {
    unsafe {
        CPU.decrement_timers();
    }
}