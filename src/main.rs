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


#[macro_use]
extern crate lazy_static;
extern crate minifb;

use std::{io::Read, fs::File};

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640*2;
const HEIGHT: usize = 360*2;

use cpu::CPU;

mod color;
mod cpu;
mod ram;
mod instructions;
mod assembler;
mod frame_buffer;
mod io;
use ram::*;
use assembler::*;

use crate::color::Color;

struct VM {
    ram: RAM,
    cpu: CPU,
}

fn on_update(vm: &mut VM, window: &mut Window) -> bool { 
    for _ in 0..50000 {
        vm.cpu.step(&mut vm.ram);
    }

    let win_width  = vm.cpu.window.width;
    let win_height = vm.cpu.window.height;
    let win_offset = vm.cpu.window.offset;

    let colors = [Color::from(0xff0d1137), Color::from(0xffe52165)];

    let frame_buffer = frame_buffer::FrameBuffer::new(&vm.ram.buff[win_offset as usize..], win_width, win_height, &colors[..]);
    if win_width == 0 || win_height == 0 {
        return true;
    }

    window
        .update_with_buffer(&frame_buffer.data, win_width as usize, win_height as usize)
        .unwrap();

    true
}

fn load_bin(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents).unwrap();
    file_contents
}
 
fn main() {
    let mut ram = RAM::new();
    let cpu = CPU::new();

    //let bin = assemble("res/prg/test.lb");
    //let bin = assemble("res/prg/add.lb");
    //let bin = assemble("res/prg/ISA_configs.Ludde_output.lb");
    let bin = assemble("res/prg/gol.lb");

    ram.set_multiple(0x00, &bin[..]);
    let mut vm = VM {ram, cpu};

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        on_update(&mut vm, &mut window);
    }
}