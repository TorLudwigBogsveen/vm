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
use ram::*;
use assembler::*;

use crate::color::Color;

struct VM {
    ram: RAM,
    cpu: CPU,
    time: f32,
}

fn on_update(vm: &mut VM, window: &mut Window) -> bool { 
    const time_step: f32 = 0.01;

    //self.time += dt;
    //if self.time >= time_step {
        //self.time -= time_step;
        for _ in 0..4000 {
            vm.cpu.step(&mut vm.ram);
            let v1 = *vm.cpu.reg_16(0);
            let v2 = *vm.cpu.reg_16(1);
            let v3 = *vm.cpu.reg_16(2);
            let v4 = *vm.cpu.reg_16(3);
            //println!("{}, {}, {}, {}", v1, v2, v3, v4);
        }
    //}
    let win_width  = vm.cpu.window.width;
    let win_height = vm.cpu.window.height;
    let win_offset = vm.cpu.window.offset;

    let colors = [Color::new(0.2, 0.2, 0.2, 1.0), Color::new(0.8, 0.8, 0.8, 1.0)];

    let mut colors = Vec::new();
    for i in 0..2 {
        colors.push(Color::from((i*256*100)));
    }

    let frame_buffer = frame_buffer::FrameBuffer::new(&vm.ram.buff[win_offset as usize..], win_width, win_height, &colors[..]);
    if win_width == 0 || win_height == 0 {
        return true;
    }
    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
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
    /*let cols = [Color::from(0xFF_FF_FF_FF), Color::from(0x00_00_00_FF), Color::from(0x00_AA_00_FF)];
    let data = [
        0b00101101, 
        0b11111111
    ];
    let frame_buffer = frame_buffer::FrameBuffer::new(&data, 0, 2, 2, &cols);
    for i in 0..(frame_buffer.data.len()/4) {
        println!("r: {}, g: {} b: {}", frame_buffer.data[i*4+0], frame_buffer.data[i*4+1], frame_buffer.data[i*4+2]);
    }*/

    let mut ram = RAM::new();
    let cpu = CPU::new();

    let bin = assemble("res/prg/test.lb");
    //let bin = assemble("res/prg/ISA_configs.Ludde_output.lb");
    for b in bin.iter().enumerate() {
        //println!("{} : {}", b.0, b.1cargo );
    }
    println!("{}", bin.len());

    ram.set_multiple(0x00, &bin[..]);
    let mut vm = VM {ram, cpu, time: 0.0};

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        on_update(&mut vm, &mut window);
        
    }
}