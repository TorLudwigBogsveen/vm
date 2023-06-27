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

use std::{fs::File, io::Read};

use cpu::CPU;

mod assembler;
mod color;
mod cpu;
mod frame_buffer;
mod instructions;
mod io;
mod ram;
use assembler::*;
use eframe::{egui::{self, TextureOptions, Window, accesskit::Vec2}, epaint::ColorImage};
use ram::*;

use crate::color::Color;

struct VM {
    ram: RAM,
    cpu: CPU,
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
    let mut vm = VM { ram, cpu };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc, vm))),
    );
}

struct MyEguiApp {
    vm: VM,
    continue_all: bool,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>, vm: VM) -> Self {
        Self {
            vm,
            continue_all: false,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        for _ in 0..100000 {
            self.vm.cpu.step(&mut self.vm.ram);
        }

        let win_width = self.vm.cpu.window.width;
        let win_height = self.vm.cpu.window.height;
        let win_offset = self.vm.cpu.window.offset;

        let colors = [Color::from(0x0d1137ff), Color::from(0xe52165ff)];

        let frame_buffer = frame_buffer::FrameBuffer::new(
            &self.vm.ram.buff[win_offset as usize..],
            win_width,
            win_height,
            &colors[..],
        );

        let image = ColorImage::from(frame_buffer);
        let texture = ctx.load_texture("framebuffer", image, TextureOptions::NEAREST);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(&texture,[640.0, 360.0]);

            if self.continue_all { 
                self.vm.cpu.flags.halted = false;
            }

            if self.vm.cpu.flags.halted && !self.continue_all {
                if ui.button("Continue").clicked() {
                    self.vm.cpu.flags.halted = false;
                }

                if ui.button("Continue all").clicked() {
                    self.continue_all = true;
                }
            }

            for (name, value) in self.vm.cpu.registers() {
                ui.label(format!("{} : {:#04x} \t {}", name, value, value));
            }
        });

        ctx.request_repaint();
    }
}
