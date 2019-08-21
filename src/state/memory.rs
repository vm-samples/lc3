use libc;
use nix::sys::{
    select::{select, FdSet},
    time::{TimeVal, TimeValLike},
};
use std::io::{self, Read};

pub struct Memory {
    memory: [u16; u16::max_value() as usize],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: [0; u16::max_value() as usize],
        }
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MemoryMappedRegister::KBSR as u16 {
            if check_key() {
                self.memory[MemoryMappedRegister::KBSR as usize] = 1 << 15;
                self.memory[MemoryMappedRegister::KBDR as usize] = get_char();
            } else {
                self.memory[MemoryMappedRegister::KBSR as usize] = 0;
            }
        }

        if address < u16::max_value() {
            self.memory[address as usize]
        } else {
            0
        }
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }
}

fn check_key() -> bool {
    let mut readfds = FdSet::new();
    readfds.insert(libc::STDIN_FILENO);

    match select(None, &mut readfds, None, None, &mut TimeVal::zero()) {
        Ok(value) => value == 1,
        Err(_) => false,
    }
}

fn get_char() -> u16 {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();

    u16::from(buffer[0])
}

enum MemoryMappedRegister {
    KBSR = 0xfe00, // keyboard status register
    KBDR = 0xfe02, // keyboard data register
}
