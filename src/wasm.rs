use cpu::Cpu;
use display::Display;
use keypad::Keypad;
use rand::{ComplementaryMultiplyWithCarryGen, CMWC_CYCLE};

// TODO: change to a constructor
static mut CPU: Cpu = Cpu {
    i: 0,
    pc: 0,
    dt: 0,
    memory: [0; 4096],
    v: [0; 16],
    display: Display {
        memory: [0; 2048]
    },
    keypad: Keypad {
        keys: [false; 16]
    },
    stack: [0; 16],
    sp: 0,
    rand: ComplementaryMultiplyWithCarryGen {
        q: [0; CMWC_CYCLE],
        c: 0,
        i: 0
    }
};

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