/*
 *   Copyright (c) 2020 Ludwig Bogsveen
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

use crate::ram::*;
use crate::instructions::*;
pub struct Flags {
    pub not_zero: bool,
    pub less_then: bool,
    pub larger_then: bool,
    pub equals: bool,
    pub owerflow: bool,
    pub underflow: bool,
    pub halted: bool,
    pub carry: bool,
}
 
impl Flags {
    pub fn new() -> Flags {
        Flags {
        not_zero: false,
        less_then: false,
        larger_then: false,
        equals: false,
        owerflow: false,
        underflow: false,
        halted: false,
        carry: false,
        }
    }
}

#[derive(Copy)]
union Register {
    full: u16,
    high_low: [u8; 2],  
}

impl Clone for Register {
    fn clone(&self) -> Self { 
        Self {
            full: unsafe { self.full }
        }
    }
}

impl Register {
    fn full(&self) -> u16 {
        unsafe { self.full }
    }

    fn high_low(&self) -> [u8; 2] {
        unsafe { self.high_low }
    }

    fn high(&self) -> u8 {
        unsafe { self.high_low[0] }
    }

    fn low(&self) -> u8 {
        unsafe { self.high_low[0] }
    }
}

#[derive(Clone, Copy)]
struct Registers {
    regs: [Register; 256]
}

impl Registers {
    fn new() -> Registers {
        Registers {
            regs: [Register{full:0}; 256]
        }
    }

    fn reg_8(&mut self, reg: u8) -> &mut u8 {
        if reg % 2 == 0 {
            unsafe { &mut self.regs[reg as usize / 2].high_low[1] }
        }
        else {
            unsafe { &mut self.regs[reg as usize / 2].high_low[0] }
        }
    }

    fn reg_16(&mut self, reg: u8) -> &mut u16 {
        unsafe { &mut self.regs[reg as usize].full }
    }
}

pub struct Window {
    pub width: u16,
    pub height: u16,
    pub offset: u16,
}
 
pub struct CPU {
    registers: Registers,
    pub instruction_ptr: u8,
    pub stack_ptr: u8,
    pub flags: Flags,
    pub window: Window,
}
 
impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            instruction_ptr: 6,
            stack_ptr: 7,
            flags: Flags::new(),
            window: Window {width: 0, height: 0, offset: 0}
        }
    }

    pub fn step(&mut self, ram: &mut RAM) {
        if self.flags.halted {
            //println!("zzz");
            //std::thread::sleep_ms(1000);
            //self.flags.halted = false;
            return;
        }

        let instruction = ram.get(self.ins_ptr(1));
        //println!("instruction: {} location: {}", instruction, self.ins_ptr(0)-1);

        match unsafe { std::mem::transmute(instruction) } {
            Instruction::NOP    => {},
            Instruction::MOVRR  => movrr (self, ram),
            Instruction::MOVRM  => movrm (self, ram),
            Instruction::MOVMR  => movmr (self, ram),
            Instruction::MOVMM  => movmm (self, ram),
            Instruction::MOVIR  => movir (self, ram),
            Instruction::MOVIM  => movim (self, ram),
            Instruction::MOVRP  => movrp (self, ram),
            Instruction::MOVPR  => movpr (self, ram),
            Instruction::MOVDRR => movdrr (self, ram),
            Instruction::MOVDRM => movdrm (self, ram),
            Instruction::MOVDMR => movdmr (self, ram),
            Instruction::MOVDMM => movdmm (self, ram),
            Instruction::MOVDIR => movdir (self, ram),
            Instruction::MOVDIM => movdim (self, ram),
            Instruction::MOVDRP => movdrp (self, ram),
            Instruction::MOVDPR => movdpr (self, ram),
            Instruction::INC    => inc (self, ram),
            Instruction::DEC    => dec (self, ram),
            Instruction::INCD   => incd(self, ram),
            Instruction::DECD   => decd(self, ram),
            Instruction::JMP    => jmp (self, ram),
            Instruction::JNE    => jne (self, ram),
            Instruction::JE     => je  (self, ram),
            Instruction::JG     => jg  (self, ram),
            Instruction::JL     => jl  (self, ram),
            Instruction::JC     => jc  (self, ram),
            Instruction::CMP    => cmp (self, ram),
            Instruction::CMPD   => cmpd (self, ram),
            Instruction::PUSH   => push(self, ram),
            Instruction::POP    => pop (self, ram),
            Instruction::PUSHD  => pushd(self, ram),
            Instruction::POPD   => popd(self, ram),
            Instruction::CALL   => call(self, ram),
            Instruction::CALLI  => calli(self, ram),
            Instruction::RET    => ret (self, ram),
            Instruction::INT    => int(self, ram),
            Instruction::ADD    => add(self, ram),
            Instruction::SUB    => sub(self, ram),
            Instruction::MUL    => mul(self, ram),
            Instruction::DIV    => div(self, ram),
            Instruction::OR     => or (self, ram),
            Instruction::AND    => and(self, ram),
            Instruction::XOR    => xor(self, ram),
            Instruction::NOR    => nor(self, ram),
            Instruction::NAND   =>nand(self, ram),
            Instruction::XNOR   =>xnor(self, ram),
            Instruction::SHL    => shl(self, ram),
            Instruction::SHR    => shr(self, ram),
            Instruction::ADDD   => addd(self, ram),
            Instruction::SUBD   => subd(self, ram),
            Instruction::MULD   => muld(self, ram),
            Instruction::DIVD   => divd(self, ram),
            Instruction::ORD    => ord(self, ram),
            Instruction::ANDD   => andd(self, ram),
            Instruction::XORD   => xord(self, ram),
            Instruction::NORD   => nord(self, ram),
            Instruction::NANDD  =>nandd(self, ram),
            Instruction::XNORD  =>xnord(self, ram),
            Instruction::SHLD   => shld(self, ram),
            Instruction::SHRD   => shrd(self, ram),
            Instruction::OUT    => out(self, ram),
            Instruction::BRK    => self.flags.halted = true,
            _ => { 
                println!("ERROR INVALID INSTRUCTION [{}] at [{}]",
                instruction,
                *self.reg_16(self.instruction_ptr)-1);   
                self.flags.halted = true;
            }
        }
    }

    pub fn reg_8(&mut self, reg: u8) -> &mut u8 {
        self.registers.reg_8(reg)
    }

    pub fn reg_16(&mut self, reg: u8) -> &mut u16 {
        self.registers.reg_16(reg)
    }

    pub fn ins_ptr(&mut self, addvance: u16) -> u16 {
        let ins_ptr = *self.reg_16(self.instruction_ptr);
        *self.reg_16(self.instruction_ptr) = self.reg_16(self.instruction_ptr).wrapping_add(addvance);
        ins_ptr
    }

    /*pub fn draw_registers(&self, gfx: &mut Graphics) {
        let mut offset = -1.0;
        for (i, reg) in self.registers.regs.iter().enumerate() {
            //gfx.draw_string(&format!("{}:{}", i, reg.full()), 0.0, offset);
            offset += 0.25;
        }
    }*/
}