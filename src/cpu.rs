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

const FLAG_NOT_ZERO: u32 = 0b00000001;
const FLAG_LESS_THAN: u32 = 0b00000010;
const FLAG_LARGER_THAN: u32 = 0b00000100;
const FLAG_EQUALS: u32 = 0b00001000;
const FLAG_OVERFLOW: u32 = 0b00010000;
const FLAG_UNDERFLOW: u32 = 0b00100000;
const FLAG_HALTED: u32 = 0b01000000;
const FLAG_CARRY: u32 = 0b10000000;
 
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

impl From<u32> for Flags {
    fn from(value: u32) -> Self {
        Flags {
            not_zero: (value & FLAG_NOT_ZERO) != 0,
            less_then: (value & FLAG_LESS_THAN) != 0,
            larger_then: (value & FLAG_LARGER_THAN) != 0,
            equals: (value & FLAG_EQUALS) != 0,
            owerflow: (value & FLAG_OVERFLOW) != 0,
            underflow: (value & FLAG_UNDERFLOW) != 0,
            halted: (value & FLAG_HALTED) != 0,
            carry: (value & FLAG_CARRY) != 0,
        }
    }
}

impl From<Flags> for u32 {
    fn from(flags: Flags) -> Self {
        let mut value = 0;
        if flags.not_zero { value |= FLAG_NOT_ZERO; }
        if flags.less_then { value |= FLAG_LESS_THAN; }
        if flags.larger_then { value |= FLAG_LARGER_THAN; }
        if flags.equals { value |= FLAG_EQUALS; }
        if flags.owerflow { value |= FLAG_OVERFLOW; }
        if flags.underflow { value |= FLAG_UNDERFLOW; }
        if flags.halted { value |= FLAG_HALTED; }
        if flags.carry { value |= FLAG_CARRY; }
        value
    }
}

#[derive(Copy, Clone)]
struct Register(u32);

impl Register {

}

enum RegisterView<'a> {
    U32(&'a u32),
    U16(&'a u16),
    U8(&'a u8),
}

impl<'a> RegisterView<'a> {
    fn as_u32(&self) -> &u32 {
        match self {
            RegisterView::U32(val) => val,
            _ => panic!("RegisterView is not a u32"),
        }
    }

    fn as_u16(&self) -> &u16 {
        match self {
            RegisterView::U16(val) => val,
            _ => panic!("RegisterView is not a u16"),
        }
    }

    fn as_u8(&self) -> &u8 {
        match self {
            RegisterView::U8(val) => val,
            _ => panic!("RegisterView is not a u8"),
        }
    }
}

enum RegisterViewMut<'a> {
    U32(&'a mut u32),
    U16(&'a mut u16),
    U8(&'a mut u8),
}

impl<'a> RegisterViewMut<'a> {
    fn as_u32(&mut self) -> &mut u32 {
        match self {
            RegisterViewMut::U32(val) => val,
            _ => panic!("RegisterView is not a u32"),
        }
    }

    fn as_u16(&mut self) -> &mut u16 {
        match self {
            RegisterViewMut::U16(val) => val,
            _ => panic!("RegisterView is not a u16"),
        }
    }

    fn as_u8(&mut self) -> &mut u8 {
        match self {
            RegisterViewMut::U8(val) => val,
            _ => panic!("RegisterView is not a u8"),
        }
    }
}

enum RegisterName {
    RIP = 0,
    RSP = 1,
    RFLG = 2,
    R0 = 3,
    R1 = 4,
    R2 = 5,
    R3 = 6,
    R4 = 7,
}

impl From<u8> for RegisterName {
    fn from(value: u8) -> Self {
        match value {
            0 => RegisterName::RIP,
            1 => RegisterName::RSP,
            2 => RegisterName::RFLG,
            3 => RegisterName::R0,
            4 => RegisterName::R1,
            5 => RegisterName::R2,
            6 => RegisterName::R3,
            7 => RegisterName::R4,
            _ => panic!("Invalid register name: {}", value),
        }
    }
}

impl From<RegisterName> for u8 {
    fn from(reg: RegisterName) -> Self {
        match reg {
            RegisterName::RIP => 0,
            RegisterName::RSP => 1,
            RegisterName::RFLG => 2,
            RegisterName::R0 => 3,
            RegisterName::R1 => 4,
            RegisterName::R2 => 5,
            RegisterName::R3 => 6,
            RegisterName::R4 => 7,
        }
    }
}

#[derive(Clone, Copy)]
struct Registers {
    regs: [Register; 8]
}

impl Registers {
    fn new() -> Registers {
        Registers {
            regs: [Register(0); 8]
        }
    }

    fn view<T: Into<RegisterName>>(&self, reg: T) -> RegisterView {
        let reg_name = reg.into();
        match reg_name {
            RegisterName::RIP => RegisterView::U32(&self.regs[0].0),
            RegisterName::RSP => RegisterView::U32(&self.regs[1].0),
            RegisterName::RFLG => RegisterView::U32(&self.regs[2].0),
            RegisterName::R0 => RegisterView::U32(&self.regs[3].0),
            RegisterName::R1 => RegisterView::U32(&self.regs[4].0),
            RegisterName::R2 => RegisterView::U32(&self.regs[5].0),
            RegisterName::R3 => RegisterView::U32(&self.regs[6].0),
            RegisterName::R4 => RegisterView::U32(&self.regs[7].0),
        }
    }

    fn view_mut<T: Into<RegisterName>>(&mut self, reg: T) -> RegisterViewMut {
        let reg_name = reg.into();
        match reg_name {
            RegisterName::RIP => RegisterViewMut::U32(&mut self.regs[0].0),
            RegisterName::RSP => RegisterViewMut::U32(&mut self.regs[1].0),
            RegisterName::RFLG => RegisterViewMut::U32(&mut self.regs[2].0),
            RegisterName::R0 => RegisterViewMut::U32(&mut self.regs[3].0),
            RegisterName::R1 => RegisterViewMut::U32(&mut self.regs[4].0),
            RegisterName::R2 => RegisterViewMut::U32(&mut self.regs[5].0),
            RegisterName::R3 => RegisterViewMut::U32(&mut self.regs[6].0),
            RegisterName::R4 => RegisterViewMut::U32(&mut self.regs[7].0),
        }
    }
}
 
pub struct CPU {
    registers: Registers,
}
 
impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }

    pub fn step(&mut self, ram: &mut RAM) {
        let flags = self.flags();
        if flags.halted {
            return;
        }

        let instruction = ram.get(*self.registers.view(RegisterName::RIP).as_u32(), 1)[0];
        let instruction = if let Ok(instruction) = Instruction::try_from(instruction) {
            instruction
        } else {
            println!("ERROR INVALID INSTRUCTION [{}] at [{}]",
            instruction,
            *self.registers.view(RegisterName::RIP).as_u32());   
            self.halt();
            return;
        };
        

        match instruction {
            Instruction::NOP    => {},
            Instruction::MOVRR  => movrr (self, ram),
            Instruction::MOVRM  => movrm (self, ram),
            Instruction::MOVMR  => movmr (self, ram),
            Instruction::MOVMM  => movmm (self, ram),
            Instruction::MOVIR  => movir (self, ram),
            Instruction::MOVIM  => movim (self, ram),
            Instruction::MOVRP  => movrp (self, ram),
            Instruction::MOVPR  => movpr (self, ram),
            Instruction::INC    => inc (self, ram),
            Instruction::DEC    => dec (self, ram),
            Instruction::JMP    => jmp (self, ram),
            Instruction::JNE    => jne (self, ram),
            Instruction::JE     => je  (self, ram),
            Instruction::JG     => jg  (self, ram),
            Instruction::JL     => jl  (self, ram),
            Instruction::JC     => jc  (self, ram),
            Instruction::CMP    => cmp (self, ram),
            Instruction::PUSH   => push(self, ram),
            Instruction::POP    => pop (self, ram),
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
            Instruction::BRK    => self.halt(),
        }
    }

    fn has_halted(&self) -> bool {
        let flags = self.flags();
        flags.halted
    }

    fn halt(&mut self) {
        let mut f = self.flags(); 
        f.halted = true; 
        self.set_flags(f);
    }

    fn flags(&self) -> Flags {
        let flags_u32 = *self.registers.view(RegisterName::RFLG).as_u32();
        let flags = Flags::from(flags_u32);
        flags
    }

    fn set_flags(&mut self, flags: Flags) {
        let flags_u32 = u32::from(flags);
        *self.registers.view_mut(RegisterName::RFLG).as_u32() = flags_u32;
    }

    fn rip(&self) -> u32 {
        *self.registers.view(RegisterName::RIP).as_u32()
    }

    pub fn rip_advance(&mut self, amount: u32) -> u32 {
        let current_rip = self.rip();
        *self.registers.view_mut(RegisterName::RIP).as_u32() = current_rip + amount;
        current_rip
    }

    /*pub fn draw_registers(&self, gfx: &mut Graphics) {
        let mut offset = -1.0;
        for (i, reg) in self.registers.regs.iter().enumerate() {
            //gfx.draw_string(&format!("{}:{}", i, reg.full()), 0.0, offset);
            offset += 0.25;
        }
    }*/
}