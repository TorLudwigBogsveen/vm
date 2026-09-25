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

use crate::cpu::*;
use crate::ram::*;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, IntoPrimitive, TryFromPrimitive)]
pub enum Instruction {
  NOP,
  BRK,
  MOVRR,
  MOVRM,
  MOVMR,
  MOVMM,
  MOVIR,
  MOVIM,
  MOVRP,
  MOVPR,
  INC,
  DEC,
  CMP,
  JMP,
  JNE,
  JE,
  JG,
  JL,
  JC,
  PUSH,
  POP,
  CALL,
  CALLI,
  RET,
  INT,
  ADD,
  SUB,
  MUL,
  DIV,
  OR,
  AND,
  XOR,
  NOR,
  NAND,
  XNOR,
  SHR,
  SHL,
}

pub fn shl(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  (*cpu.reg_8(ram.get(dst))) <<= *cpu.reg_8(ram.get(src));
}

pub fn shr(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  (*cpu.reg_8(ram.get(dst))) >>= *cpu.reg_8(ram.get(src));
}

pub fn add(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  let res = (*cpu.reg_8(ram.get(dst))).overflowing_add(*cpu.reg_8(ram.get(src)));
  *cpu.reg_8(ram.get(dst)) = res.0;
  cpu.flags.carry = res.1;
}

pub fn sub(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  let res = (*cpu.reg_8(ram.get(dst))).overflowing_sub(*cpu.reg_8(ram.get(src)));
  *cpu.reg_8(ram.get(dst)) = res.0;
  cpu.flags.carry = res.1;
}

pub fn mul(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  let res = (*cpu.reg_8(ram.get(dst))).overflowing_mul(*cpu.reg_8(ram.get(src)));
  *cpu.reg_8(ram.get(dst)) = res.0;
  cpu.flags.carry = res.1;
}

pub fn div(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  let res = (*cpu.reg_8(ram.get(dst))).overflowing_div(*cpu.reg_8(ram.get(src)));
  *cpu.reg_8(ram.get(dst)) = res.0;
  cpu.flags.carry = res.1;
}

pub fn or(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  *cpu.reg_8(ram.get(dst)) |= *cpu.reg_8(ram.get(src));
}

pub fn and(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  *cpu.reg_8(ram.get(dst)) &= *cpu.reg_8(ram.get(src));
}

pub fn xor(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  *cpu.reg_8(ram.get(dst)) ^= *cpu.reg_8(ram.get(src));
}

pub fn nor(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  let res = *cpu.reg_8(ram.get(dst)) | *cpu.reg_8(ram.get(src));
  *cpu.reg_8(ram.get(dst)) = !res;
}

pub fn nand(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  let res = *cpu.reg_8(ram.get(dst)) & *cpu.reg_8(ram.get(src));
  *cpu.reg_8(ram.get(dst)) = !res;
}

pub fn xnor(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  let res = *cpu.reg_8(ram.get(dst)) ^ *cpu.reg_8(ram.get(src));
  *cpu.reg_8(ram.get(dst)) = !res;
}

pub fn movrr(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  *cpu.reg_8(ram.get(dst)) = *cpu.reg_8(ram.get(src));
}

pub fn movrm(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = ram.get_ptr(cpu.rip_advance(2));
  ram.set(dst, *cpu.reg_8(ram.get(src)));
}

pub fn movmr(cpu: &mut CPU, ram: &mut RAM) {
  let src = ram.get_ptr(cpu.rip_advance(2));
  let dst = cpu.rip_advance(1);
  *cpu.reg_8(ram.get(dst)) = ram.get(src);
}

pub fn movmm(cpu: &mut CPU, ram: &mut RAM) {
  let src = ram.get_ptr(cpu.rip_advance(2));
  let dst = ram.get_ptr(cpu.rip_advance(2));
  ram.set(dst, ram.get(src));
}

pub fn movir(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = cpu.rip_advance(1);
  *cpu.reg_8(ram.get(dst)) = ram.get(src);
}

pub fn movim(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = ram.get_ptr(cpu.rip_advance(2));
  ram.set(dst, ram.get(src));
}

pub fn movrp(cpu: &mut CPU, ram: &mut RAM) {
  let src = cpu.rip_advance(1);
  let dst = ram.get(cpu.rip_advance(1));
  ram.set(*cpu.reg_16(dst), *cpu.reg_8(ram.get(src)));

  //let a = *cpu.reg_16(dst);
  //println!("{} = {}", a, ram.get(a));
}

pub fn movpr(cpu: &mut CPU, ram: &mut RAM) {
  let src = ram.get(cpu.rip_advance(1));
  let dst = cpu.rip_advance(1);
  *cpu.reg_8(ram.get(dst)) = ram.get(*cpu.reg_16(src));
}

pub fn inc(cpu: &mut CPU, ram: &mut RAM) {
  let dst = ram.get(cpu.rip_advance(1));
  match (*cpu.reg_8(dst)).checked_add(1) {
    Some(_) => {},
    None => {
      cpu.flags.owerflow = true;
      cpu.flags.carry = true;
    }
  }
  *cpu.reg_8(dst) = (*cpu.reg_8(dst)).wrapping_add(1);
}

pub fn dec(cpu: &mut CPU, ram: &mut RAM) {
  let dst = ram.get(cpu.rip_advance(1));
  match (*cpu.reg_8(dst)).checked_sub(1) {
    Some(_) => {},
    None => {
      cpu.flags.underflow = true;
    }
  }
  *cpu.reg_8(dst) = (*cpu.reg_8(dst)).wrapping_sub(1);
}

pub fn cmp(cpu: &mut CPU, ram: &mut RAM) {
  let lhs = ram.get(cpu.rip_advance(1));
  let rhs = ram.get(cpu.rip_advance(1));
  let lhs = *cpu.reg_8(lhs) as i16;
  let rhs = *cpu.reg_8(rhs) as i16;

  let diff = lhs - rhs;
  if diff < 0 {
    cpu.flags.less_then = true;
    cpu.flags.larger_then = false;
    cpu.flags.not_zero = true;
    cpu.flags.equals = false;
  } else if diff > 0 {
    cpu.flags.less_then = false;
    cpu.flags.larger_then = true;
    cpu.flags.not_zero = true;
    cpu.flags.equals = false;
  }
  else {
    cpu.flags.less_then = false;
    cpu.flags.larger_then = false;
    cpu.flags.not_zero = false;
    cpu.flags.equals = true;
  }
}

pub fn jmp(cpu: &mut CPU, ram: &mut RAM) {
  // println!("{}", cpu.rip_advance(0));
  let rip_advance = cpu.rip_advance(1);
  *cpu.reg_16(cpu.instruction_ptr) = *cpu.reg_16(ram.get(rip_advance));
  //println!("num: {}", *cpu.reg_16(ram.get(rip_advance)));
  //std::thread::sleep(Duration::from_millis(1000));
  //println!("JMP");
}

pub fn jne(cpu: &mut CPU, ram: &mut RAM) {
  if !cpu.flags.equals {
    jmp(cpu, ram);
  } else {
    cpu.rip_advance(1);
  }
}

pub fn je(cpu: &mut CPU, ram: &mut RAM) {
  if cpu.flags.equals {
    jmp(cpu, ram);
  } else {
    cpu.rip_advance(1);
  }
}

pub fn jg(cpu: &mut CPU, ram: &mut RAM) {
  if cpu.flags.larger_then {
    jmp(cpu, ram);
  } else {
    cpu.rip_advance(1);
  }
}

pub fn jl(cpu: &mut CPU, ram: &mut RAM) {
  if cpu.flags.less_then {
    jmp(cpu, ram);
  } else {
    cpu.rip_advance(1);
  }
}

pub fn jc(cpu: &mut CPU, ram: &mut RAM) {
  if cpu.flags.owerflow {
    jmp(cpu, ram);
    cpu.flags.owerflow = false;
  } else {
    cpu.rip_advance(1);
  }
}

pub fn push(cpu: &mut CPU, ram: &mut RAM) {
  *cpu.reg_16(cpu.stack_ptr) -= 1;
  let rip_advance = cpu.rip_advance(1);
  let src = *cpu.reg_8(ram.get(rip_advance));
  let dst = *cpu.reg_16(cpu.stack_ptr);
  ram.set(dst, src);
}

pub fn pop(cpu: &mut CPU, ram: &mut RAM) {
  let src = ram.get(*cpu.reg_16(cpu.stack_ptr));
  let dst = ram.get(cpu.rip_advance(1));
  *cpu.reg_8(dst) = src;
  *cpu.reg_16(cpu.stack_ptr) += 1;
}

pub fn call(cpu: &mut CPU, ram: &mut RAM) {
  *cpu.reg_16(cpu.stack_ptr) = (*cpu.reg_16(cpu.stack_ptr)).overflowing_sub(2).0;
  let rip_advance = cpu.rip_advance(1);
  let sub_addr = *cpu.reg_16(ram.get(rip_advance));
  ram.set_ptr(*cpu.reg_16(cpu.stack_ptr), cpu.rip_advance(0));
  *cpu.reg_16(cpu.instruction_ptr) = sub_addr;
}

pub fn calli(cpu: &mut CPU, ram: &mut RAM) {
  *cpu.reg_16(cpu.stack_ptr) = (*cpu.reg_16(cpu.stack_ptr)).overflowing_sub(2).0;
  let sub_addr = ram.get_ptr(cpu.rip_advance(2));
  ram.set_ptr(*cpu.reg_16(cpu.stack_ptr), cpu.rip_advance(0));
  *cpu.reg_16(cpu.instruction_ptr) = sub_addr;
}

pub fn ret(cpu: &mut CPU, ram: &mut RAM) {
  let ret_addr = ram.get_ptr(*cpu.reg_16(cpu.stack_ptr));
  *cpu.reg_16(cpu.instruction_ptr) = ret_addr;
  *cpu.reg_16(cpu.stack_ptr) += 2;
}

pub fn int(cpu: &mut CPU, ram: &mut RAM) {
  let rip_advance = cpu.rip_advance(1);
  let interrupt_code = cpu.reg_8(ram.get(rip_advance));
  match interrupt_code {
    0 => {
    }
    _ => {
      for i in 0..6 {
        let src = *cpu.reg_16(i);
        let dst = *cpu.reg_16(cpu.stack_ptr);
        ram.set_ptr(dst, src);
        *cpu.reg_16(cpu.stack_ptr) -= 2;
      }
    }
  }
}